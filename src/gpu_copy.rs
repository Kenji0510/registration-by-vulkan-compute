use anyhow::{Context, Result};
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
    sync::{self, GpuFuture},
};

use crate::{
    gpu_transform::TransformGpuContext, gpu_voxel::VoxelGpuContext, init_gpu::VulkanContext,
};

pub fn copy_d_to_h(
    vulkan_context: &VulkanContext,
    voxel_gpu_context: &VoxelGpuContext,
    // cov_gpu_context: &CovarianceGpuContext,
    transform_gpu_context: &TransformGpuContext,
    // ) -> Result<(Vec<[f32; 3]>, Vec<[f32; 9]>)> {
) -> Result<Vec<[f32; 3]>> {
    // <!--- Copy the transformed points from GPU to CPU --->
    let queue = &vulkan_context.queue;
    let command_buffer_allocator = &vulkan_context.command_buffer_allocator;

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue.queue_family_index().clone(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .context("Failed to create command buffer builder")?;

    let copy_output_pts_src = transform_gpu_context
        .d_buf_output_pts
        .as_ref()
        .context("Failed to get output points buffer for copy")?
        .clone()
        .slice(0..(voxel_gpu_context.h_downsampled_pts_num * 3) as u64);
    let copy_output_pts_dst = transform_gpu_context
        .staging_buf_output_pts
        .as_ref()
        .context("Failed to get staging output points buffer")?
        .clone()
        .slice(0..(voxel_gpu_context.h_downsampled_pts_num * 3) as u64);

    command_buffer_builder.copy_buffer(CopyBufferInfo::buffers(
        copy_output_pts_src,
        copy_output_pts_dst,
    ))?;

    let command_buffer = command_buffer_builder.build()?;
    let future = sync::now(vulkan_context.device.clone())
        .then_execute(queue.clone(), command_buffer)?
        .then_signal_fence_and_flush()?;
    future.wait(None)?;

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
    // <!--- Copy the transformed points from GPU to CPU --->

    Ok(output_points)
}
