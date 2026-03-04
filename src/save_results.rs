use anyhow::{Context, Result};
use log::debug;
use ndarray::Array2;

use crate::{
    gpu_copy::copy_d_to_h, gpu_transform::TransformParams, init_gpu::VulkanContext, oprate_pcd::{
        PointXYZ, convert_pcd_xyz_to_xyz_color, convert_vecf32_to_pcd_xyz, save_pcd_with_color,
        save_xyz_pcd,
    }, registration::GpuContexts
};

pub fn save_results(
    vulkan_context: &VulkanContext,
    gpu_contexts: &mut GpuContexts,
    transform_params: &TransformParams,
    source_pcd: &Vec<PointXYZ>,
    target_pcd: &Vec<PointXYZ>,
    max_iterations: usize,
    label: &str,
) -> Result<()> {
    // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->
    gpu_contexts
        .transform_gpu_ctx_source
        .transform(
            &gpu_contexts.voxel_gpu_ctx_source,
            &gpu_contexts.covariance_gpu_ctx_source,
            *transform_params,
        )
        .context("Failed to apply final transformation to source points using GPU")?;

    let transformed_source_pts = copy_d_to_h(
        &vulkan_context,
        &gpu_contexts.voxel_gpu_ctx_source,
        &gpu_contexts.transform_gpu_ctx_source,
    )
    .context("Failed to copy transformed source points from GPU to CPU")?;

    let transformed_source_pts_vecf32: Vec<[f32; 3]> = transformed_source_pts
        .iter()
        .map(|p| [p[0], p[1], p[2]])
        .collect();

    let source_pcd_with_color = convert_pcd_xyz_to_xyz_color(&source_pcd, (255, 0, 0)); // Red
    let aligned_source_pcd = convert_vecf32_to_pcd_xyz(&transformed_source_pts_vecf32);
    let aligned_source_pcd_with_color =
        convert_pcd_xyz_to_xyz_color(&aligned_source_pcd, (0, 255, 0)); // Green
    let target_pcd_with_color = convert_pcd_xyz_to_xyz_color(target_pcd, (0, 0, 255)); // Blue
    let mut aligned_source_and_target = aligned_source_pcd_with_color.clone();
    aligned_source_and_target.extend_from_slice(&target_pcd_with_color);
    aligned_source_and_target.extend_from_slice(&source_pcd_with_color);
    let aligned_save_path = format!(
        "data/output/debug/integrate-reverse-pattern/aligned-source-and-target_{}_iter-{}.pcd",
        label, max_iterations
    );
    debug!(
        "Saving aligned source and target point cloud to: {}",
        aligned_save_path
    );
    save_pcd_with_color(&aligned_source_and_target, &aligned_save_path)
        .context("Failed to save aligned source and target point cloud")?;

    // <!--- Apply the final transformation to the original source point cloud and save the aligned point cloud --->

    Ok(())
}
