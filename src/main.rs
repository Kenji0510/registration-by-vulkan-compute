use anyhow::{Context, Result};
use log::{debug, info};
use registration_vulkan::{
    gpu_copy::copy_d_to_h,
    gpu_covariance::CovarianceGpuContext,
    gpu_search_neighbor::{PtsInfo, SearchGpuContext},
    gpu_transform::{TransformGpuContext, TransformParams},
    gpu_voxel::VoxelGpuContext,
    init_gpu::{GpuBuffer, VulkanContext},
    oprate_pcd::{convert_vecf32_to_pcd_xyz_covs, load_pcd_xyz, save_pcd_with_covs},
    transform_data::pcd_to_vecf32,
};

// const SOURCE_PCD_PATH: &str = "data/input/H927/vggt-data_output_voxel_025_xyz_only.pcd";
const SOURCE_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd"; // For test
const TARGET_PCD_PATH: &str = "data/input/H927/lab-room_voxel_025_xyz_only.pcd";

const VOXEL_SIZE: f32 = 0.25;

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
    let search_params = PtsInfo {
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

    Ok(())
}
