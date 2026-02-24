use anyhow::{Context, Result};
use log::{debug, info};
use ndarray::Array2;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    instance::debug,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
};

use crate::{
    gpu_covariance::CovarianceGpuContext, gpu_transform::TransformGpuContext,
    gpu_voxel::VoxelGpuContext, init_gpu::VulkanContext,
};

pub fn copy_d_to_h(
    // vulkan_context: &VulkanContext,
    voxel_gpu_context: &VoxelGpuContext,
    // cov_gpu_context: &CovarianceGpuContext,
    transform_gpu_context: &TransformGpuContext,
) -> Result<(Vec<[f32; 3]>, Vec<[f32; 9]>)> {
    // let staging_buf_output_pts = Some(Buffer::new_slice::<f32>(
    //     vulkan_context.memory_allocator.clone(),
    //     BufferCreateInfo {
    //         usage: BufferUsage::TRANSFER_DST,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         memory_type_filter: MemoryTypeFilter::PREFER_HOST
    //             | MemoryTypeFilter::HOST_RANDOM_ACCESS,
    //         ..Default::default()
    //     },
    //     (voxel_gpu_context.h_downsampled_pts_num * 3) as u64,
    // )?);

    // let staging_buf_output_covs = Some(Buffer::new_slice::<f32>(
    //     vulkan_context.memory_allocator.clone(),
    //     BufferCreateInfo {
    //         usage: BufferUsage::TRANSFER_DST,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo {
    //         memory_type_filter: MemoryTypeFilter::PREFER_HOST
    //             | MemoryTypeFilter::HOST_RANDOM_ACCESS,
    //         ..Default::default()
    //     },
    //     (voxel_gpu_context.h_downsampled_pts_num * 9) as u64,
    // )?);

    // let start = std::time::Instant::now();
    // // Copy the downsampled points from the GPU to the CPU
    // let out_pts_content = voxel_gpu_context
    //     .staging_buf_output_pts
    //     .as_ref()
    //     .context("staging_buf_output_pts is None")?
    //     .read()?;

    // let output_points: Vec<[f32; 3]> = out_pts_content
    //     .chunks_exact(3)
    //     .take(voxel_gpu_context.h_downsampled_pts_num)
    //     .map(|chunk| -> Result<[f32; 3]> {
    //         chunk.try_into().context("Failed to map for output points")
    //     })
    //     .collect::<Result<Vec<_>, _>>()?;

    // // Copy the covariance of downsampled points from the GPU to the CPU
    // let out_pts_covs_content = cov_gpu_context
    //     .staging_buf_output_covs
    //     .as_ref()
    //     .context("Failed to get staging output covariances buffer")?
    //     .read()?;
    // let output_covs: Vec<[f32; 9]> = out_pts_covs_content
    //     .chunks_exact(9)
    //     .take(voxel_gpu_context.h_downsampled_pts_num)
    //     .map(|chunk| {
    //         chunk
    //             .try_into()
    //             .expect("Chunk should have exactly 9 elements")
    //     })
    //     .collect();

    // let duration = start.elapsed();
    // debug!("Time taken to copy data from GPU to CPU: {:?}", duration);

    // <!--- DEBUG --->
    let out_pts_content = transform_gpu_context
        .staging_buf_output_pts
        .as_ref()
        .context("Failed to get staging output points buffer")?
        .read()?;
    let output_points: Vec<[f32; 3]> = out_pts_content
        .chunks_exact(3)
        .take(voxel_gpu_context.h_downsampled_pts_num)
        .map(|chunk| -> Result<[f32; 3]> {
            chunk.try_into().context("Failed to map for output points")
        })
        .collect::<Result<Vec<_>, _>>()?;

    let out_covs_content = transform_gpu_context
        .staging_buf_output_covs
        .as_ref()
        .context("Failed to get staging output covariances buffer")?
        .read()?;
    let output_covs: Vec<[f32; 9]> = out_covs_content
        .chunks_exact(9)
        .take(voxel_gpu_context.h_downsampled_pts_num)
        .map(|chunk| -> Result<[f32; 9]> {
            chunk
                .try_into()
                .context("Failed to map for output covariances")
        })
        .collect::<Result<Vec<_>, _>>()?;
    // <!--- DEBUG --->

    Ok((output_points, output_covs))
}
