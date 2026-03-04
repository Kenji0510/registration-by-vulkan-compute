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
    registration::{GpuContexts, calculate_target_center, mat4_mul, registration_icp},
    reverse_pattern::{
        create_fb_flip_matrix, create_lr_flip_matrix, create_original_matrix,
        create_rot_90_x_matrix, create_rot_90_y_matrix, create_rot_90_z_matrix,
        create_rot_180_x_matrix, create_rot_180_y_matrix, create_rot_180_z_matrix,
        create_rot_minus_90_x_matrix, create_rot_minus_90_y_matrix, create_rot_minus_90_z_matrix,
        create_ud_flip_matrix,
    },
    save_results::save_results,
    transform_data::pcd_to_vecf32,
};
use vulkano::instance::debug;

const SOURCE_PCD_PATH: &str = "data/input/H927/vggt-data_output_voxel_025_xyz_only.pcd";
// const SOURCE_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd"; // For test
const TARGET_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd";

const VOXEL_SIZE: f32 = 0.5;
const MAX_DIST_SQ: f32 = 10.0;
const MIN_RMSE: f32 = VOXEL_SIZE * 0.5;
const MAX_ITERATIONS: usize = 40;

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

    let mut gpu_contexts = GpuContexts {
        voxel_gpu_ctx_source: &mut gpu_voxel_ctx_for_source,
        voxel_gpu_ctx_target: &mut gpu_voxel_ctx_for_target,
        covariance_gpu_ctx_source: &mut gpu_covariance_ctx_for_source,
        covariance_gpu_ctx_target: &mut gpu_covariance_ctx_for_target,
        transform_gpu_ctx_source: &mut gpu_transform_ctx_for_source,
        search_neighbor_gpu_ctx: &mut gpu_neighbor_search_ctx,
        knn_search_gpu_ctx: &mut gpu_knn_search_ctx,
        normals_gpu_ctx_target: &mut gpu_normals_ctx_for_target,
        icp_gpu_ctx: &mut gpu_icp_ctx,
    };

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

    let flip_rot_transforms = vec![
        ("Original", create_original_matrix()),
        ("LR_Flip_Reverse-Y", create_lr_flip_matrix()),
        ("UD_Flip_Reverse-Z", create_ud_flip_matrix()),
        ("Front-Back_Flip_Reverse-X", create_fb_flip_matrix()),
        ("Rot_180_Z", create_rot_180_z_matrix()),
        ("Rot_180_Y", create_rot_180_y_matrix()),
        ("Rot_180_X", create_rot_180_x_matrix()),
        ("Rot_90_X", create_rot_90_x_matrix()),
        ("Rot_-90_X", create_rot_minus_90_x_matrix()),
        ("Rot_90_Y", create_rot_90_y_matrix()),
        ("Rot_-90_Y", create_rot_minus_90_y_matrix()),
        ("Rot_90_Z", create_rot_90_z_matrix()),
        ("Rot_-90_Z", create_rot_minus_90_z_matrix()),
    ];

    let start = std::time::Instant::now();

    // <!--- Calculate the initial center transformation for source to target. With voxelization --->
    let initial_center_transform = calculate_target_center(
        &mut gpu_contexts,
        &source_pts_vec,
        &target_pts_vec,
        VOXEL_SIZE,
    )
    .context("Failed to calculate initial center transformation")?;
    // <!--- Calculate the initial center transformation for source to target. With voxelization --->

    debug!(
        "Initial center transformation matrix:\n{:?}",
        initial_center_transform
    );

    let mut registration_results = Vec::new();

    for (label, transform) in flip_rot_transforms.iter() {
        let combined_transform = mat4_mul(transform, &initial_center_transform);
        debug!(
            "Combined initial transformation with {}:\n{:?}",
            label, combined_transform
        );

        // <!--- Perform ICP iterations --->
        let (icp_matrix, rmse) = registration_icp(
            &mut gpu_contexts,
            &combined_transform,
            VOXEL_SIZE,
            MAX_ITERATIONS,
            MIN_RMSE,
        )
        .context("Failed to process ICP registration")?;
        debug!("Final transformation matrix:\n{:?}", icp_matrix);
        debug!("Final RMSE: {}", rmse);
        // <!--- Perform ICP iterations --->

        registration_results.push((label.to_string(), icp_matrix, rmse));
    }

    let elapsed = start.elapsed();
    info!("Total registration time: {:.2?}", elapsed);

    registration_results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

    info!("=== Registration Results ===");
    for (label, icp_matrix, rmse) in registration_results.iter() {
        // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->
        let final_transform_params = TransformParams {
            r00: icp_matrix[[0, 0]],
            r01: icp_matrix[[0, 1]],
            r02: icp_matrix[[0, 2]],
            r10: icp_matrix[[1, 0]],
            r11: icp_matrix[[1, 1]],
            r12: icp_matrix[[1, 2]],
            r20: icp_matrix[[2, 0]],
            r21: icp_matrix[[2, 1]],
            r22: icp_matrix[[2, 2]],
            t0: icp_matrix[[0, 3]],
            t1: icp_matrix[[1, 3]],
            t2: icp_matrix[[2, 3]],
            num_points: gpu_contexts.voxel_gpu_ctx_source.h_downsampled_pts_num as u32,
        };

        save_results(
            &vulkan_context,
            &mut gpu_contexts,
            &final_transform_params,
            &source_pcd,
            &target_pcd,
            MAX_ITERATIONS,
            label,
        )
        .context("Failed to save results")?;
        // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->

        info!("Saved results for transformation: {}", label);
        info!("Final RMSE for transformation {}: {}", label, rmse);
        info!("ICP transformation matrix for {}:\n{:?}", label, icp_matrix);
    }
    info!("=== Registration Results ===");

    let best_registration = registration_results
        .iter()
        .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    if let Some((best_label, best_icp_matrix, best_rmse)) = best_registration {
        info!("Best registration result: {}", best_label);
        info!("Best RMSE: {}", best_rmse);
        info!("Best ICP transformation matrix:\n{:?}", best_icp_matrix);
    } else {
        info!("No valid registration results found.");
    }

    Ok(())
}
