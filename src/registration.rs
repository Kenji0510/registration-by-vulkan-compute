use anyhow::{Context, Result};
use log::{debug, info};
use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::Solve;

use crate::{
    gpu_covariance::CovarianceGpuContext,
    gpu_icp::{IcpGpuContext, IcpStaticBuffers},
    gpu_knn_search::KnnSearchGpuContext,
    gpu_normals::{NormalParams, NormalsGpuContext},
    gpu_search_neighbor::SearchGpuContext,
    gpu_transform::{TransformGpuContext, TransformParams},
    gpu_voxel::VoxelGpuContext,
};

pub struct GpuContexts<'a> {
    pub voxel_gpu_ctx_source: &'a mut VoxelGpuContext,
    pub voxel_gpu_ctx_target: &'a mut VoxelGpuContext,
    pub covariance_gpu_ctx_source: &'a mut CovarianceGpuContext,
    pub covariance_gpu_ctx_target: &'a mut CovarianceGpuContext,
    pub transform_gpu_ctx_source: &'a mut TransformGpuContext,
    pub search_neighbor_gpu_ctx: &'a mut SearchGpuContext,
    pub knn_search_gpu_ctx: &'a mut KnnSearchGpuContext,
    pub normals_gpu_ctx_target: &'a mut NormalsGpuContext,
    pub icp_gpu_ctx: &'a mut IcpGpuContext,
    // source_pts_vec: &'a Vec<[f32; 3]>,
    // target_pts_vec: &'a Vec<[f32; 3]>,
}

pub fn registration_icp(
    gpu_contexts: &mut GpuContexts,
    initial_center_transform: &Array2<f32>,
    voxel_size: f32,
    max_iterations: usize,
    min_rmse: f32,
) -> Result<(Array2<f32>, f32)> {
    let downsampled_source_pts_num = gpu_contexts.voxel_gpu_ctx_source.h_downsampled_pts_num;
    let downsampled_target_pts_num = gpu_contexts.voxel_gpu_ctx_target.h_downsampled_pts_num;

    // <!--- Compute the covariance of the point clouds using GPU voxelization --->
    let d_source_covs = gpu_contexts
        .covariance_gpu_ctx_source
        .compute_covariances(
            &gpu_contexts.voxel_gpu_ctx_source,
            downsampled_source_pts_num,
            false,
        )
        .context("Failed to compute covariances for source")?;
    let d_target_covs = gpu_contexts
        .covariance_gpu_ctx_target
        .compute_covariances(
            &gpu_contexts.voxel_gpu_ctx_target,
            downsampled_target_pts_num,
            false,
        )
        .context("Failed to compute covariances for target")?;
    // <!--- Compute the covariance of the point clouds using GPU voxelization --->

    // <!--- Compute the transformation of the point clouds using GPU voxelization --->
    let transform_params_source = TransformParams {
        r00: initial_center_transform[[0, 0]],
        r01: initial_center_transform[[0, 1]],
        r02: initial_center_transform[[0, 2]],
        r10: initial_center_transform[[1, 0]],
        r11: initial_center_transform[[1, 1]],
        r12: initial_center_transform[[1, 2]],
        r20: initial_center_transform[[2, 0]],
        r21: initial_center_transform[[2, 1]],
        r22: initial_center_transform[[2, 2]],
        t0: initial_center_transform[[0, 3]],
        t1: initial_center_transform[[1, 3]],
        t2: initial_center_transform[[2, 3]],
        num_points: downsampled_source_pts_num as u32,
    };
    gpu_contexts
        .transform_gpu_ctx_source
        .transform(
            &gpu_contexts.voxel_gpu_ctx_source,
            &gpu_contexts.covariance_gpu_ctx_source,
            transform_params_source,
        )
        .context("Failed to transform points and covariances for source")?;
    // <!--- Compute the transformation of the point clouds using GPU voxelization --->

    // <!--- Perform knn neighbor search using GPU --->
    gpu_contexts
        .knn_search_gpu_ctx
        .knn_search_neighbors(
            &gpu_contexts.voxel_gpu_ctx_target,
            downsampled_target_pts_num,
        )
        .context("Failed to perform knn neighbor search using GPU")?;
    // <!--- Perform knn neighbor search using GPU --->

    // <!--- Compute normals using GPU --->
    let normal_params = NormalParams {
        num_points: downsampled_target_pts_num as u32,
        vp_x: 0.0,
        vp_y: 0.0,
        vp_z: 0.0,
    };
    let target_normals = gpu_contexts
        .normals_gpu_ctx_target
        .compute_normals(
            &gpu_contexts.voxel_gpu_ctx_target,
            downsampled_target_pts_num,
            &gpu_contexts.knn_search_gpu_ctx,
            normal_params,
        )
        .context("Failed to compute normals using GPU")?;

    // <!--- DEBUG --->
    // let target_pts_with_normals =
    //     combine_pts_with_normals(&downsampled_target_pts, &target_normals)?;
    // let debug_save_path_normals = "data/output/debug/target_normals.pcd";
    // save_pcd(&target_pts_with_normals, debug_save_path_normals)
    //     .context("Failed to save target points with normals")?;
    // <!--- DEBUG --->
    // <!--- Compute normals using GPU --->

    let icp_static_bufs = IcpStaticBuffers {
        d_source_pts: gpu_contexts
            .transform_gpu_ctx_source
            .d_buf_output_pts
            .as_ref()
            .context("source pts not allocated")?
            .clone(),
        d_target_pts: gpu_contexts
            .voxel_gpu_ctx_target
            .d_buf_out_pts
            .as_ref()
            .context("target pts not allocated")?
            .clone(),
        d_target_normals: gpu_contexts
            .normals_gpu_ctx_target
            .d_buf_normals
            .as_ref()
            .context("target normals not allocated")?
            .clone(),
    };

    let mut current_transform = initial_center_transform.clone();
    let mut rmse = 0.0f32;
    let mut prev_rmse = f32::MAX;

    debug!("Starting ICP iterations...");

    // <!--- Perform ICP iterations --->
    for iter in 0..max_iterations {
        debug!("--- ICP Iteration {} ---", iter + 1);

        // let current_max_dist = if iter < (max_iterations / 2) as usize {
        //     3.0
        // } else if iter < (max_iterations * 3 / 4) as usize {
        //     1.0
        // } else {
        //     voxel_size * 2.0
        // };
        // let current_max_dist_sq = current_max_dist * current_max_dist;
        let current_max_dist_sq = 5.0;

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
        gpu_contexts
            .transform_gpu_ctx_source
            .transform(
                &gpu_contexts.voxel_gpu_ctx_source,
                &gpu_contexts.covariance_gpu_ctx_source,
                transform_params_source,
            )
            .context("Failed to transform points and covariances for source")?;
        // <!--- Compute the transformation of the point clouds using GPU voxelization --->

        // <!--- Perform neighbor search using GPU --->
        let (neighbor_indices, neighbor_distances) = gpu_contexts
            .search_neighbor_gpu_ctx
            .search_neighbor(
                &gpu_contexts.transform_gpu_ctx_source,
                downsampled_source_pts_num,
                &gpu_contexts.voxel_gpu_ctx_target,
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

        rmse = if cnt > 0 {
            (sum / cnt as f32).sqrt()
        } else {
            0.0
        };
        debug!("Iter {}: RMSE = {:.6}, Valid points = {}", iter, rmse, cnt);

        // if (prev_rmse - rmse).abs() < TOLERANCE {
        if rmse < min_rmse {
            info!("Converged! (RMSE difference is less than threshold)");
            break;
        }
        prev_rmse = rmse;
        // <!--- Check convergence (RMSE) --->

        // <!--- Compute ICP using GPU --->
        let (h_H, h_b) = gpu_contexts
            .icp_gpu_ctx
            .compute_icp(
                &icp_static_bufs,
                &gpu_contexts.search_neighbor_gpu_ctx,
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

    Ok((current_transform, rmse))
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

pub fn mat4_mul(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
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

pub fn calculate_target_center(
    gpu_contexts: &mut GpuContexts,
    source_pts_vec: &Vec<[f32; 3]>,
    target_pts_vec: &Vec<[f32; 3]>,
    voxel_size: f32,
) -> Result<Array2<f32>> {
    // <!--- Downsample the point clouds using GPU voxelization --->
    let downsampled_source_pts = gpu_contexts
        .voxel_gpu_ctx_source
        .voxelization(&source_pts_vec, source_pts_vec.len(), voxel_size)
        .context("Failed to compute voxelization for source")?;
    let downsampled_source_pts_num = downsampled_source_pts.len();
    let downsampled_target_pts = gpu_contexts
        .voxel_gpu_ctx_target
        .voxelization(&target_pts_vec, target_pts_vec.len(), voxel_size)
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

    // <!--- Calculate the centroids of the downsampled point clouds and translate them to the origin --->

    Ok(initial_transform)
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

pub fn convert_vecf32_to_arr2_f32(pts: &[[f32; 3]]) -> Array2<f32> {
    let mut arr = Array2::<f32>::zeros((pts.len(), 3));
    for (i, point) in pts.iter().enumerate() {
        arr[[i, 0]] = point[0];
        arr[[i, 1]] = point[1];
        arr[[i, 2]] = point[2];
    }
    arr
}
