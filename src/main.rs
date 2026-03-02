use core::f32;

use anyhow::{Context, Result};
use log::{debug, info};
use ndarray::{Array1, Array2};
use ndarray_linalg::Solve;
use registration_vulkan::{
    gpu_copy::copy_d_to_h,
    gpu_covariance::CovarianceGpuContext,
    gpu_icp::{IcpGpuContext, IcpParams},
    gpu_knn_search::{KnnSearchConsts, KnnSearchGpuContext},
    gpu_normals::{NormalParams, NormalsGpuContext, combine_pts_with_normals},
    gpu_search_neighbor::{SearchGpuContext, SearchNeighborConsts},
    gpu_transform::{TransformGpuContext, TransformParams},
    gpu_voxel::VoxelGpuContext,
    init_gpu::VulkanContext,
    oprate_pcd::{convert_vecf32_to_pcd_xyz_covs, load_pcd_xyz, save_pcd, save_pcd_with_covs},
    transform_data::pcd_to_vecf32,
};

// const SOURCE_PCD_PATH: &str = "data/input/H927/vggt-data_output_voxel_025_xyz_only.pcd";
const SOURCE_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd"; // For test
const TARGET_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd";

const VOXEL_SIZE: f32 = 0.25;
const TOLERANCE: f32 = VOXEL_SIZE * VOXEL_SIZE;

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

    // let gpu_buffer = GpuBuffer {
    //     voxel_d_buf_source_pts: None,
    //     voxel_d_buf_source_pts_num: 0,
    //     voxel_d_buf_source_covs: None,
    //     voxel_d_buf_target_pts: None,
    //     voxel_d_buf_target_pts_num: 0,
    //     voxel_d_buf_target_covs: None,
    // };

    let source_pcd = load_pcd_xyz(SOURCE_PCD_PATH).context("Failed to load the source pcd")?;
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
    let mut downsampled_source_pts = gpu_voxel_ctx_for_source
        .voxelization(&source_pts_vec, source_pts_vec.len(), VOXEL_SIZE)
        .context("Failed to compute voxelization for source")?;
    let mut downsampled_target_pts = gpu_voxel_ctx_for_target
        .voxelization(&target_pts_vec, target_pts_vec.len(), VOXEL_SIZE)
        .context("Failed to compute voxelization for target")?;
    debug!(
        "Downsampled points num of source: {}",
        downsampled_source_pts.len()
    );
    debug!(
        "Downsampled points num of target: {}",
        downsampled_target_pts.len()
    );
    // <!--- Downsample the point clouds using GPU voxelization --->

    // <!--- Compute the covariance of the point clouds using GPU voxelization --->
    let d_source_covs = gpu_covariance_ctx_for_source
        .compute_covariances(
            &gpu_voxel_ctx_for_source,
            &downsampled_source_pts,
            downsampled_source_pts.len(),
            false,
        )
        .context("Failed to compute covariances for source")?;
    let d_target_covs = gpu_covariance_ctx_for_target
        .compute_covariances(
            &gpu_voxel_ctx_for_target,
            &downsampled_target_pts,
            downsampled_target_pts.len(),
            false,
        )
        .context("Failed to compute covariances for target")?;
    // <!--- Compute the covariance of the point clouds using GPU voxelization --->

    // <!--- Compute the transformation of the point clouds using GPU voxelization --->
    // Z軸基準で180度回転
    // let transform_params_source = TransformParams {
    //     r00: -1.0,
    //     r01: 0.0,
    //     r02: 0.0,
    //     r10: 0.0,
    //     r11: -1.0,
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
        r00: 1.0,
        r01: 0.0,
        r02: 0.0,
        r10: 0.0,
        r11: 1.0,
        r12: 0.0,
        r20: 0.0,
        r21: 0.0,
        r22: 1.0,
        t0: 0.0,
        t1: 0.0,
        t2: 0.0,
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
    let (h_downsampled_source_pts, h_downsampled_source_covs) = copy_d_to_h(
        // &vulkan_context,
        &gpu_voxel_ctx_for_source,
        &gpu_transform_ctx_for_source,
    )
    .context("Failed to copy downsampled points and covariances from GPU to CPU")?;
    // <!--- Copy the downsampled points and their covariances from GPU to CPU --->

    // <!--- DEBUG --->
    let downsampled_source_pts_with_covs =
        convert_vecf32_to_pcd_xyz_covs(&h_downsampled_source_pts, &h_downsampled_source_covs);
    let debug_save_path = "data/output/debug/downsampled_source_with_covs.pcd";
    debug!(
        "Saving downsampled source points with covariances to: {}",
        debug_save_path
    );
    save_pcd_with_covs(&downsampled_source_pts_with_covs, debug_save_path)
        .context("Failed to save downsampled source points with covariances")?;
    // <!--- DEBUG --->

    // <!--- Perform neighbor search using GPU --->
    let search_params = SearchNeighborConsts {
        num_source: gpu_transform_ctx_for_source.num_points as u32,
        num_target: gpu_voxel_ctx_for_target.h_downsampled_pts_num as u32,
    };

    let (neighbor_indices, neighbor_distances) = gpu_neighbor_search_ctx
        .search_neighbor(
            &gpu_transform_ctx_for_source,
            &gpu_voxel_ctx_for_target,
            search_params,
        )
        .context("Failed to perform neighbor search using GPU")?;

    // <!--- DEBUG --->
    // println!("Neighbor indices: {:?}", neighbor_indices);
    // println!("Neighbor distances: {:?}", neighbor_distances);
    // <!--- DEBUG --->

    // <!--- Perform neighbor search using GPU --->

    // <!--- Perform knn neighbor search using GPU --->
    let knn_search_params = KnnSearchConsts {
        num_points: gpu_transform_ctx_for_source.num_points as u32,
    };
    gpu_knn_search_ctx
        .knn_search_neighbors(&gpu_voxel_ctx_for_target, knn_search_params)
        .context("Failed to perform knn neighbor search using GPU")?;

    // <!--- Perform knn neighbor search using GPU --->

    // <!--- Compute normals using GPU --->
    let normal_params = NormalParams {
        num_points: gpu_voxel_ctx_for_target.h_downsampled_pts_num as u32,
        vp_x: 0.0,
        vp_y: 0.0,
        vp_z: 0.0,
    };
    let target_normals = gpu_normals_ctx_for_target
        .compute_normals(
            &gpu_voxel_ctx_for_target,
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

    // <!--- Compute ICP using GPU --->
    let icp_params = IcpParams {
        num_source: gpu_transform_ctx_for_source.num_points as i32,
        num_target: gpu_voxel_ctx_for_target.h_downsampled_pts_num as i32,
        max_dist_sq: TOLERANCE,
    };
    let (h_H, h_b) = gpu_icp_ctx
        .compute_icp(
            &gpu_transform_ctx_for_source,
            icp_params.num_source as usize,
            &gpu_voxel_ctx_for_target,
            icp_params.num_target as usize,
            &gpu_normals_ctx_for_target,
            &gpu_neighbor_search_ctx,
            icp_params.max_dist_sq,
        )
        .context("Failed to compute ICP using GPU")?;
    // <!--- DEBUG --->
    println!("H (6x6 matrix):");
    for i in 0..6 {
        println!("{:.6} {:.6} {:.6} {:.6} {:.6} {:.6}", h_H[i * 6], h_H[i * 6 + 1], h_H[i * 6 + 2], h_H[i * 6 + 3], h_H[i * 6 + 4], h_H[i * 6 + 5]);
    }
    println!("b (6x1 vector):");
    for i in 0..6 {
        println!("{:.6}", h_b[i]);
    }
    // <!--- DEBUG --->

    let h_matrix = Array2::from_shape_vec((6, 6), h_H.iter().map(|&v| v as f64).collect())?;
    let b_vector = Array1::from_shape_vec(6, h_b.iter().map(|&v| v as f64).collect())?;
    let delta_matrix = solve_linear_system_6x6(h_matrix, b_vector)?;

    println!("Delta transformation matrix:");
    for i in 0..4 {
        println!("{:.6} {:.6} {:.6} {:.6}", delta_matrix[[i, 0]], delta_matrix[[i, 1]], delta_matrix[[i, 2]], delta_matrix[[i, 3]]);
    }

    // Check convergence (RMSE)
    let mut sum = 0.0f32;
    let mut cnt = 0usize;
    let mut rmse = 0.0f32;

    for j in 0..transform_params_source.num_points as usize {
        let idx = neighbor_indices[j];
        if idx < 0 {
            continue;
        }
        if neighbor_distances[j] > TOLERANCE {
            continue;
        }
        sum += neighbor_distances[j];
        cnt += 1;
    }
    rmse = (sum / cnt as f32).sqrt();
    debug!("RMSE: {}, count of valid correspondences: {}", rmse, cnt);
    // <!--- Compute ICP using GPU --->

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