use core::f32;
use std::char::MAX;

use anyhow::{Context, Result};
use log::{debug, info};
use ndarray::{Array1, Array2, Axis, s};
use ndarray_linalg::Solve;
use registration_vulkan::{
    gpu_copy::copy_d_to_h,
    gpu_covariance::CovarianceGpuContext,
    gpu_icp::{IcpGpuContext, IcpStaticBuffers},
    gpu_knn_search::{KnnSearchConsts, KnnSearchGpuContext},
    gpu_normals::{NormalParams, NormalsGpuContext, combine_pts_with_normals},
    gpu_search_neighbor::{SearchGpuContext, SearchNeighborParams},
    gpu_transform::{TransformGpuContext, TransformParams},
    gpu_voxel::VoxelGpuContext,
    init_gpu::VulkanContext,
    oprate_pcd::{
        PointXYZ, convert_vecf32_to_pcd_xyz, convert_vecf32_to_pcd_xyz_covs, load_pcd_xyz,
        save_pcd, save_pcd_with_covs, save_xyz_pcd,
    },
    transform_data::pcd_to_vecf32,
};
use vulkano::instance::debug;

const SOURCE_PCD_PATH: &str = "data/input/H927/vggt-data_output_voxel_025_xyz_only.pcd";
// const SOURCE_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd"; // For test
const TARGET_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd";

const VOXEL_SIZE: f32 = 0.25;
const MAX_DIST_SQ: f32 = 10.0;
const MIN_RMSE: f32 = VOXEL_SIZE * 0.5;
const MAX_ITERATIONS: usize = 20;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let vulkan_context = VulkanContext::new().context("Failed to initialize Vulkan context")?;
    let mut gpu_voxel_ctx_for_source = VoxelGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU voxel context for source")?;
    let mut gpu_voxel_ctx_for_target = VoxelGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU voxel context for target")?;
    let mut gpu_covariance_ctx_for_source = CovarianceGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU covariance context for source")?;
    let mut gpu_covariance_ctx_for_target = CovarianceGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU covariance context for target")?;
    let mut gpu_transform_ctx_for_source = TransformGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU transform context for source")?;
    let mut gpu_neighbor_search_ctx = SearchGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU neighbor search context")?;
    let mut gpu_knn_search_ctx = KnnSearchGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU knn search context")?;
    let mut gpu_normals_ctx_for_target = NormalsGpuContext::new(vulkan_context.clone())
        .context("Failed to create GPU normals context for target")?;
    let mut gpu_icp_ctx =
        IcpGpuContext::new(vulkan_context.clone()).context("Failed to create GPU ICP context")?;

    let source_pcd = load_pcd_xyz(SOURCE_PCD_PATH).context("Failed to load the source pcd")?;;
    let target_pcd = load_pcd_xyz(TARGET_PCD_PATH).context("Failed to load the target pcd")?;

    info!("=== Parameters ===");
    info!("Source PCD path: {}", SOURCE_PCD_PATH);
    info!("Target PCD path: {}", TARGET_PCD_PATH);
    info!("Voxel size: {}", VOXEL_SIZE);
    info!("====================");

    let source_pts_vec = pcd_to_vecf32(&source_pcd);
    let target_pts_vec = pcd_to_vecf32(&target_pcd);
    info!("Points num of source: {}", source_pts_vec.len());
    info!("Points num of target: {}", target_pts_vec.len());

    // <!--- Downsample the point clouds using GPU voxelization --->
    let downsampled_source_pts = gpu_voxel_ctx_for_source
        .voxelization(&source_pts_vec, source_pts_vec.len(), VOXEL_SIZE)
        .context("Failed to compute voxelization for source")?;
    let downsampled_source_pts_num = downsampled_source_pts.len();
    let downsampled_target_pts = gpu_voxel_ctx_for_target
        .voxelization(&target_pts_vec, target_pts_vec.len(), VOXEL_SIZE)
        .context("Failed to compute voxelization for target")?;
    let downsampled_target_pts_num = downsampled_target_pts.len();

    debug!(
        "Downsampled points num of source: {}",
        downsampled_source_pts_num
    );
    debug!(
        "Downsampled points num of target: {}",
        downsampled_target_pts_num
    );
    // <!--- Downsample the point clouds using GPU voxelization --->

    // <!--- Calculate the centroids of the downsampled point clouds and translate them to the origin --->
    let downsampled_source_arr = convert_vecf32_to_arr2_f32(&downsampled_source_pts);
    let downsampled_target_arr = convert_vecf32_to_arr2_f32(&downsampled_target_pts);
    let initial_transform =
        registration_pcd_center(&downsampled_source_arr, &downsampled_target_arr);

    debug!("Source centroid translation:\n{}", initial_transform);
    // <!--- Calculate the centroids of the downsampled point clouds and translate them to the origin --->

    // <!--- Compute the covariance of the point clouds using GPU voxelization --->
    let d_source_covs = gpu_covariance_ctx_for_source
        .compute_covariances(
            &gpu_voxel_ctx_for_source,
            &downsampled_source_pts,
            downsampled_source_pts_num,
            false,
        )
        .context("Failed to compute covariances for source")?;
    let d_target_covs = gpu_covariance_ctx_for_target
        .compute_covariances(
            &gpu_voxel_ctx_for_target,
            &downsampled_target_pts,
            downsampled_target_pts_num,
            false,
        )
        .context("Failed to compute covariances for target")?;
    // <!--- Compute the covariance of the point clouds using GPU voxelization --->

    // <!--- Compute the transformation of the point clouds using GPU voxelization --->
    // Z軸基準で30度回転
    // let transform_params_source = TransformParams {
    //     r00: 0.8660254,
    //     r01: 0.5,
    //     r02: 0.0,
    //     r10: -0.5,
    //     r11: 0.8660254,
    //     r12: 0.0,
    //     r20: 0.0,
    //     r21: 0.0,
    //     r22: 1.0,
    //     t0: 0.0,
    //     t1: 0.0,
    //     t2: 0.0,
    //     num_points: downsampled_source_pts.len() as u32,
    // };
    let transform_params_source = TransformParams {
        r00: initial_transform[[0, 0]],
        r01: initial_transform[[0, 1]],
        r02: initial_transform[[0, 2]],
        r10: initial_transform[[1, 0]],
        r11: initial_transform[[1, 1]],
        r12: initial_transform[[1, 2]],
        r20: initial_transform[[2, 0]],
        r21: initial_transform[[2, 1]],
        r22: initial_transform[[2, 2]],
        t0: initial_transform[[0, 3]],
        t1: initial_transform[[1, 3]],
        t2: initial_transform[[2, 3]],
        num_points: downsampled_source_pts.len() as u32,
    };
    gpu_transform_ctx_for_source
        .transform(
            &gpu_voxel_ctx_for_source,
            &gpu_covariance_ctx_for_source,
            transform_params_source,
        )
        .context("Failed to transform points and covariances for source")?;
    // <!--- Compute the transformation of the point clouds using GPU voxelization --->

    // <!--- Copy the downsampled points and their covariances from GPU to CPU --->
    // let (h_downsampled_source_pts, h_downsampled_source_covs) = copy_d_to_h(
    //     // &vulkan_context,
    //     &gpu_voxel_ctx_for_source,
    //     &gpu_transform_ctx_for_source,
    // )
    // .context("Failed to copy downsampled points and covariances from GPU to CPU")?;
    // // <!--- Copy the downsampled points and their covariances from GPU to CPU --->

    // // <!--- DEBUG --->
    // let downsampled_source_pts_with_covs =
    //     convert_vecf32_to_pcd_xyz_covs(&h_downsampled_source_pts, &h_downsampled_source_covs);
    // let debug_save_path = "data/output/debug/downsampled_source_with_covs.pcd";
    // debug!(
    //     "Saving downsampled source points with covariances to: {}",
    //     debug_save_path
    // );
    // save_pcd_with_covs(&downsampled_source_pts_with_covs, debug_save_path)
    //     .context("Failed to save downsampled source points with covariances")?;
    // <!--- DEBUG --->

    // <!--- Perform knn neighbor search using GPU --->
    gpu_knn_search_ctx
        .knn_search_neighbors(&gpu_voxel_ctx_for_target, downsampled_target_pts_num)
        .context("Failed to perform knn neighbor search using GPU")?;
    // <!--- Perform knn neighbor search using GPU --->

    // <!--- Compute normals using GPU --->
    let normal_params = NormalParams {
        num_points: downsampled_target_pts_num as u32,
        vp_x: 0.0,
        vp_y: 0.0,
        vp_z: 0.0,
    };
    let target_normals = gpu_normals_ctx_for_target
        .compute_normals(
            &gpu_voxel_ctx_for_target,
            downsampled_target_pts_num,
            &gpu_knn_search_ctx,
            normal_params,
        )
        .context("Failed to compute normals using GPU")?;

    // <!--- DEBUG --->
    let target_pts_with_normals =
        combine_pts_with_normals(&downsampled_target_pts, &target_normals)?;
    let debug_save_path_normals = "data/output/debug/target_normals.pcd";
    save_pcd(&target_pts_with_normals, debug_save_path_normals)
        .context("Failed to save target points with normals")?;
    // <!--- DEBUG --->
    // <!--- Compute normals using GPU --->

    let icp_static_bufs = IcpStaticBuffers {
        d_source_pts: gpu_transform_ctx_for_source
            .d_buf_output_pts
            .as_ref()
            .context("source pts not allocated")?
            .clone(),
        d_target_pts: gpu_voxel_ctx_for_target
            .d_buf_out_pts
            .as_ref()
            .context("target pts not allocated")?
            .clone(),
        d_target_normals: gpu_normals_ctx_for_target
            .d_buf_normals
            .as_ref()
            .context("target normals not allocated")?
            .clone(),
    };

    // let mut current_transform = ndarray::array![
    //     // [0.8660254, -0.5, 0.0, 0.0],
    //     // [0.5, 0.8660254, 0.0, 0.0],
    //     // [0.0, 0.0, 1.0, 0.0],
    //     // [0.0, 0.0, 0.0, 1.0]
    //     [1.0, 0.0, 0.0, 0.0],
    //     [0.0, 1.0, 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0, 0.0, 0.0, 1.0]
    // ];
    let mut current_transform = initial_transform.clone();
    let mut prev_rmse = f32::MAX;

    debug!("Starting ICP iterations...");

    // <!--- Perform ICP iterations --->
    for iter in 0..MAX_ITERATIONS {
        debug!("--- ICP Iteration {} ---", iter + 1);

        let current_max_dist = if iter < 10 {
            3.0
        } else if iter < 15 {
            1.0
        } else {
            VOXEL_SIZE * 2.0
        };
        let current_max_dist_sq = current_max_dist * current_max_dist;

        let transform_params_source = TransformParams {
            r00: current_transform[[0, 0]],
            r01: current_transform[[0, 1]],
            r02: current_transform[[0, 2]],
            r10: current_transform[[1, 0]],
            r11: current_transform[[1, 1]],
            r12: current_transform[[1, 2]],
            r20: current_transform[[2, 0]],
            r21: current_transform[[2, 1]],
            r22: current_transform[[2, 2]],
            t0: current_transform[[0, 3]],
            t1: current_transform[[1, 3]],
            t2: current_transform[[2, 3]],
            num_points: downsampled_source_pts_num as u32,
        };

        // <!--- Compute the transformation of the point clouds using GPU voxelization --->
        gpu_transform_ctx_for_source
            .transform(
                &gpu_voxel_ctx_for_source,
                &gpu_covariance_ctx_for_source,
                transform_params_source,
            )
            .context("Failed to transform points and covariances for source")?;
        // <!--- Compute the transformation of the point clouds using GPU voxelization --->

        // <!--- Perform neighbor search using GPU --->
        let (neighbor_indices, neighbor_distances) = gpu_neighbor_search_ctx
            .search_neighbor(
                &gpu_transform_ctx_for_source,
                downsampled_source_pts_num,
                &gpu_voxel_ctx_for_target,
                downsampled_target_pts_num,
            )
            .context("Failed to perform neighbor search using GPU")?;

        // <!--- DEBUG --->
        // println!("Neighbor indices: {:?}", neighbor_indices);
        // println!("Neighbor distances: {:?}", neighbor_distances);
        // <!--- DEBUG --->

        // <!--- Perform neighbor search using GPU --->

        // <!--- Check convergence (RMSE) --->
        let mut sum = 0.0f32;
        let mut cnt = 0usize;
        for j in 0..downsampled_source_pts_num {
            let idx = neighbor_indices[j];
            if idx < 0 || neighbor_distances[j] > current_max_dist_sq {
                continue;
            }
            sum += neighbor_distances[j];
            cnt += 1;
        }

        let rmse = if cnt > 0 {
            (sum / cnt as f32).sqrt()
        } else {
            0.0
        };
        debug!("Iter {}: RMSE = {:.6}, Valid points = {}", iter, rmse, cnt);

        // if (prev_rmse - rmse).abs() < TOLERANCE {
        if rmse < MIN_RMSE {
            info!("Converged! (RMSE difference is less than threshold)");
            break;
        }
        prev_rmse = rmse;
        // <!--- Check convergence (RMSE) --->

        // <!--- Compute ICP using GPU --->
        let (h_H, h_b) = gpu_icp_ctx
            .compute_icp(
                &icp_static_bufs,
                &gpu_neighbor_search_ctx,
                downsampled_source_pts_num as usize,
                downsampled_target_pts_num as usize,
                current_max_dist_sq,
            )
            .context("Failed to compute ICP using GPU")?;

        let h_matrix = Array2::from_shape_vec((6, 6), h_H.iter().map(|&v| v as f64).collect())?;
        let b_vector = Array1::from_shape_vec(6, h_b.iter().map(|&v| v as f64).collect())?;

        let delta_matrix = match solve_linear_system_6x6(h_matrix, b_vector) {
            Ok(mat) => mat,
            Err(_) => {
                log::warn!("Linear solve failed (singular matrix). Stopping ICP.");
                break;
            }
        };
        // <!--- Compute ICP using GPU --->

        // <!--- Update the current transformation --->
        current_transform = mat4_mul(&delta_matrix, &current_transform);
        // <!--- Update the current transformation --->
    }

    debug!("Final transformation matrix:\n{:?}", current_transform);

    // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->
    let final_transform_params = TransformParams {
        r00: current_transform[[0, 0]],
        r01: current_transform[[0, 1]],
        r02: current_transform[[0, 2]],
        r10: current_transform[[1, 0]],
        r11: current_transform[[1, 1]],
        r12: current_transform[[1, 2]],
        r20: current_transform[[2, 0]],
        r21: current_transform[[2, 1]],
        r22: current_transform[[2, 2]],
        t0: current_transform[[0, 3]],
        t1: current_transform[[1, 3]],
        t2: current_transform[[2, 3]],
        num_points: downsampled_source_pts_num as u32,
    };

    gpu_transform_ctx_for_source
        .transform(
            &gpu_voxel_ctx_for_source,
            &gpu_covariance_ctx_for_source,
            final_transform_params,
        )
        .context("Failed to apply final transformation to source points using GPU")?;

    let transformed_source_pts =
        copy_d_to_h(&gpu_voxel_ctx_for_source, &gpu_transform_ctx_for_source)
            .context("Failed to copy transformed source points from GPU to CPU")?;

    let transformed_source_pts_vecf32: Vec<[f32; 3]> = transformed_source_pts
        .0
        .iter()
        .map(|p| [p[0], p[1], p[2]])
        .collect();

    let aligned_source_pcd = convert_vecf32_to_pcd_xyz(&transformed_source_pts_vecf32);
    let mut aligned_source_and_target = aligned_source_pcd.clone();
    aligned_source_and_target.extend_from_slice(&target_pcd);
    let aligned_save_path = format!(
        "data/output/debug/aligned-source-and-target_iter-{}.pcd",
        MAX_ITERATIONS
    );
    debug!(
        "Saving aligned source and target point cloud to: {}",
        aligned_save_path
    );
    save_xyz_pcd(&aligned_source_and_target, &aligned_save_path)
        .context("Failed to save aligned source and target point cloud")?;

    // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->

    Ok(())
}

fn solve_linear_system_6x6(a: Array2<f64>, b: Array1<f64>) -> Result<Array2<f32>> {
    let x = a
        .solve(&b)
        .or_else(|_| Err(anyhow::anyhow!("Linear solve failed")))?;

    // x = [alpha, beta, gamma, tx, ty, tz]
    let delta_matrix = convert_se3_to_matrix4(x);
    Ok(delta_matrix)
}

// [alpha, beta, gamma, tx, ty, tz] -> 4x4 matrix
fn convert_se3_to_matrix4(x: Array1<f64>) -> Array2<f32> {
    let alpha = x[0];
    let beta = x[1];
    let gamma = x[2];
    let tx = x[3];
    let ty = x[4];
    let tz = x[5];

    let theta = (alpha * alpha + beta * beta + gamma * gamma).sqrt();
    let r: Array2<f64>;

    if theta < 1e-9 {
        r = ndarray::array![
            [1.0, -gamma, beta],
            [gamma, 1.0, -alpha],
            [-beta, alpha, 1.0]
        ];
    } else {
        let k_x = alpha / theta;
        let k_y = beta / theta;
        let k_z = gamma / theta;
        let c = theta.cos();
        let s = theta.sin();
        let v = 1.0 - c;

        r = ndarray::array![
            [
                k_x * k_x * v + c,
                k_x * k_y * v - k_z * s,
                k_x * k_z * v + k_y * s
            ],
            [
                k_x * k_y * v + k_z * s,
                k_y * k_y * v + c,
                k_y * k_z * v - k_x * s
            ],
            [
                k_x * k_z * v - k_y * s,
                k_y * k_z * v + k_x * s,
                k_z * k_z * v + c
            ]
        ];
    }

    ndarray::array![
        [
            r[[0, 0]] as f32,
            r[[0, 1]] as f32,
            r[[0, 2]] as f32,
            tx as f32
        ],
        [
            r[[1, 0]] as f32,
            r[[1, 1]] as f32,
            r[[1, 2]] as f32,
            ty as f32
        ],
        [
            r[[2, 0]] as f32,
            r[[2, 1]] as f32,
            r[[2, 2]] as f32,
            tz as f32
        ],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

fn mat4_mul(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    // a(4x4) * b(4x4)
    let mut out = Array2::<f32>::zeros((4, 4));
    for i in 0..4 {
        for j in 0..4 {
            let mut s = 0.0f32;
            for k in 0..4 {
                s += a[[i, k]] * b[[k, j]];
            }
            out[[i, j]] = s;
        }
    }
    out
}

fn registration_pcd_center(
    source_points: &Array2<f32>,
    target_points: &Array2<f32>,
) -> Array2<f32> {
    // Calculate centroids
    let source_centroid = source_points.mean_axis(Axis(0)).unwrap();
    let target_centroid = target_points.mean_axis(Axis(0)).unwrap();

    // Calculate translation (Move source centroid to target centroid)
    let translation = &target_centroid - &source_centroid;

    // Create transformation matrix
    let mut transform = Array2::<f32>::eye(4);
    transform[[0, 3]] = translation[0];
    transform[[1, 3]] = translation[1];
    transform[[2, 3]] = translation[2];

    transform
}

fn convert_vecf32_to_arr2_f32(pts: &[[f32; 3]]) -> Array2<f32> {
    let mut arr = Array2::<f32>::zeros((pts.len(), 3));
    for (i, point) in pts.iter().enumerate() {
        arr[[i, 0]] = point[0];
        arr[[i, 1]] = point[1];
        arr[[i, 2]] = point[2];
    }
    arr
}
