# 03/03/2026
```bash
warning: unused imports: `debug` and `info`
 --> src/gpu_copy.rs:2:11
  |
2 | use log::{debug, info};
  |           ^^^^^  ^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused import: `ndarray::Array2`
 --> src/gpu_copy.rs:3:5
  |
3 | use ndarray::Array2;
  |     ^^^^^^^^^^^^^^^

warning: unused imports: `AllocationCreateInfo`, `BufferCreateInfo`, `BufferUsage`, `Buffer`, `MemoryTypeFilter`, and `instance::debug`
 --> src/gpu_copy.rs:5:14
  |
5 |     buffer::{Buffer, BufferCreateInfo, BufferUsage},
  |              ^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^^^
6 |     instance::debug,
  |     ^^^^^^^^^^^^^^^
7 |     memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
  |                         ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused imports: `gpu_covariance::CovarianceGpuContext` and `init_gpu::VulkanContext`
  --> src/gpu_copy.rs:11:5
   |
11 |     gpu_covariance::CovarianceGpuContext, gpu_transform::TransformGpuContext,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
12 |     gpu_voxel::VoxelGpuContext, init_gpu::VulkanContext,
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `core::num`
 --> src/gpu_voxel.rs:2:5
  |
2 | use core::num;
  |     ^^^^^^^^^

warning: unused import: `Context`
 --> src/oprate_pcd.rs:1:14
  |
1 | use anyhow::{Context, Result};
  |              ^^^^^^^

warning: unused import: `ndarray::Array2`
 --> src/save_results.rs:3:5
  |
3 | use ndarray::Array2;
  |     ^^^^^^^^^^^^^^^

warning: unused import: `save_xyz_pcd`
  --> src/save_results.rs:10:9
   |
10 |         save_xyz_pcd,
   |         ^^^^^^^^^^^^

warning: unused variable: `only_compute_covs`
  --> src/gpu_covariance.rs:97:9
   |
97 |         only_compute_covs: bool,
   |         ^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_only_compute_covs`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `query_pool`
   --> src/gpu_covariance.rs:221:13
    |
221 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `query_pool`
   --> src/gpu_icp.rs:282:13
    |
282 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `query_pool`
   --> src/gpu_knn_search.rs:207:13
    |
207 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `query_pool`
   --> src/gpu_normals.rs:208:13
    |
208 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `query_pool`
   --> src/gpu_search_neighbor.rs:246:13
    |
246 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `query_pool`
   --> src/gpu_transform.rs:259:13
    |
259 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `descriptor_set_allocator`
   --> src/gpu_voxel.rs:154:13
    |
154 |         let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
    |             ^^^^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_descriptor_set_allocator`

warning: unused variable: `query_pool`
   --> src/gpu_voxel.rs:591:13
    |
591 |         let query_pool = if timestamps_supported {
    |             ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_query_pool`

warning: unused variable: `d_source_covs`
  --> src/registration.rs:41:9
   |
41 |     let d_source_covs = gpu_contexts
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_d_source_covs`

warning: unused variable: `d_target_covs`
  --> src/registration.rs:49:9
   |
49 |     let d_target_covs = gpu_contexts
   |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_d_target_covs`

warning: unused variable: `target_normals`
   --> src/registration.rs:102:9
    |
102 |     let target_normals = gpu_contexts
    |         ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_target_normals`

warning: variable `prev_rmse` is assigned to, but never used
   --> src/registration.rs:143:9
    |
143 |     let mut prev_rmse = f32::MAX;
    |         ^^^^^^^^^^^^^
    |
    = note: consider using `_prev_rmse` instead

warning: value assigned to `prev_rmse` is never read
   --> src/registration.rs:229:9
    |
229 |         prev_rmse = rmse;
    |         ^^^^^^^^^^^^^^^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: field `d_buf_input_pts` is never read
  --> src/gpu_covariance.rs:30:5
   |
23 | pub struct CovarianceGpuContext {
   |            -------------------- field in this struct
...
30 |     d_buf_input_pts: Option<Subbuffer<[f32]>>,
   |     ^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `staging_h_buf_input_pts` is never read
  --> src/gpu_voxel.rs:45:5
   |
32 | pub struct VoxelGpuContext {
   |            --------------- field in this struct
...
45 |     staging_h_buf_input_pts: Option<Subbuffer<[f32]>>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^

warning: variable `h_H` should have a snake case name
   --> src/registration.rs:233:14
    |
233 |         let (h_H, h_b) = gpu_contexts
    |              ^^^ help: convert the identifier to snake case: `h_h`
    |
    = note: `#[warn(non_snake_case)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: `registration_vulkan` (lib) generated 25 warnings (run `cargo fix --lib -p registration_vulkan` to apply 20 suggestions)
warning: unused import: `std::char::MAX`
 --> src/main.rs:2:5
  |
2 | use std::char::MAX;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `Array1`, `Array2`, `Axis`, and `s`
 --> src/main.rs:6:15
  |
6 | use ndarray::{Array1, Array2, Axis, s};
  |               ^^^^^^  ^^^^^^  ^^^^  ^

warning: unused import: `ndarray_linalg::Solve`
 --> src/main.rs:7:5
  |
7 | use ndarray_linalg::Solve;
  |     ^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `IcpStaticBuffers`, `KnnSearchConsts`, `NormalParams`, `PointXYZ`, `SearchNeighborParams`, `combine_pts_with_normals`, `convert_vecf32_to_pcd_xyz_covs`, `convert_vecf32_to_pcd_xyz`, `gpu_copy::copy_d_to_h`, `save_pcd_with_covs`, `save_pcd`, and `save_xyz_pcd`
  --> src/main.rs:9:5
   |
 9 |     gpu_copy::copy_d_to_h,
   |     ^^^^^^^^^^^^^^^^^^^^^
10 |     gpu_covariance::CovarianceGpuContext,
11 |     gpu_icp::{IcpGpuContext, IcpStaticBuffers},
   |                              ^^^^^^^^^^^^^^^^
12 |     gpu_knn_search::{KnnSearchConsts, KnnSearchGpuContext},
   |                      ^^^^^^^^^^^^^^^
13 |     gpu_normals::{NormalParams, NormalsGpuContext, combine_pts_with_normals},
   |                   ^^^^^^^^^^^^                     ^^^^^^^^^^^^^^^^^^^^^^^^
14 |     gpu_search_neighbor::{SearchGpuContext, SearchNeighborParams},
   |                                             ^^^^^^^^^^^^^^^^^^^^
...
19 |         PointXYZ, convert_vecf32_to_pcd_xyz, convert_vecf32_to_pcd_xyz_covs, load_pcd_xyz,
   |         ^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
20 |         save_pcd, save_pcd_with_covs, save_xyz_pcd,
   |         ^^^^^^^^  ^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^

warning: unused import: `vulkano::instance::debug`
  --> src/main.rs:33:5
   |
33 | use vulkano::instance::debug;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^

warning: constant `MAX_DIST_SQ` is never used
  --> src/main.rs:40:7
   |
40 | const MAX_DIST_SQ: f32 = 10.0;
   |       ^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `registration_vulkan` (bin "registration_vulkan") generated 6 warnings (run `cargo fix --bin "registration_vulkan" -p registration_vulkan` to apply 5 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/registration_vulkan`
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] === Available Vulkan Devices ===
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Device 0: NVIDIA GeForce RTX 4080 (DiscreteGpu)
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Selected device: NVIDIA GeForce RTX 4080 (DiscreteGpu)
    
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] === Vulkan Device Information ===
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Device Name: NVIDIA GeForce RTX 4080
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Device Type: DiscreteGpu
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] Vulkan context initialized successfully.
[2026-03-03T12:37:06Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-03T12:37:06Z INFO  registration_vulkan] === Parameters ===
[2026-03-03T12:37:06Z INFO  registration_vulkan] Source PCD path: data/input/H927/vggt-data_output_voxel_025_xyz_only.pcd
[2026-03-03T12:37:06Z INFO  registration_vulkan] Target PCD path: data/input/H927/lab-room_voxel_025_xyz_only.pcd
[2026-03-03T12:37:06Z INFO  registration_vulkan] Voxel size: 0.25
[2026-03-03T12:37:06Z INFO  registration_vulkan] ====================
[2026-03-03T12:37:06Z INFO  registration_vulkan] Points num of source: 10134
[2026-03-03T12:37:06Z INFO  registration_vulkan] Points num of target: 7285
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 10134 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 458.376µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 7285 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 133.782µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5864
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Downsampled points num of source: 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Downsampled points num of target: 5864
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Initial center transformation matrix:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with Original:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5738 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 151.847µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5864 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 151.957µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Reallocating buffers for 5738 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 112.563µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Reallocating buffers for 5864 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 883.378µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Reallocating buffers for 5864 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 109.787µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 114.957µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Reallocating buffers for 5738 points
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 413.291µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Allocating buffers for H and b matrix
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 141.538µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 106.762µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 406.177µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 134.755µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.483µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.074µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.945µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.13µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.259µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.86µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.059µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.412µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.832µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.741µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 400.956µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.55µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.759µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.109µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 114.536µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 88.577µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.976µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.78µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.343µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.103µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.085µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.954µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.675µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.225µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.308µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.609µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.811µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 107.433µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.879µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.206µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.602µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.823µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.015µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.706µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.039µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.362µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.642µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.956µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.588µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.238µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 399.224µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 131.058µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.295µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.511µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.411µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 105.158µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.437µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.159µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.929µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.903µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.723µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.514µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.713µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.791µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559937, 0.65169847, 0.061343856, 11.157985],
     [-0.65453327, 0.753722, 0.05906883, 7.2072988],
     [-0.007741158, -0.08480724, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with LR_Flip_Reverse-Y:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, -1.0, 0.0, -9.637826],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 125.918µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 122.08µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.887µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 864.441µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 90.861µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.174µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.123µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.205µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.583µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.121µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.206µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.736µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.047µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.279µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.464µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 411.407µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 127.04µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.777µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.519µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.848µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.215µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.238µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.447µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.849µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 418.691µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.365µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.445µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.648µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.987µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.993µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.739µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.071µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.092µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.352µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.357µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.615µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.28µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.614µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 103.525µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.157µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 133.061µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.154µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.639µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.8µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.987µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.699µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.544µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.694µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.485µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.046µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.19µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.401µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.086µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.408µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.152µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.045µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.462µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.677µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.934µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.074µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.679µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.337µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.627µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.467µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.403µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559936, 0.6516985, 0.061343867, 11.157986],
     [-0.6545333, 0.7537221, 0.05906883, 7.207299],
     [-0.0077411593, -0.08480724, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with UD_Flip_Reverse-Z:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, -1.0, -0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 126.749µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 128.313µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.164µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 865.714µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 90.281µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.185µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.593µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.903µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.906µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.456µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.223µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.012µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.446µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.113µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.629µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.034µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.877µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.527µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.003µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.234µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.096µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.859µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 129.595µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.415µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.316µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.243µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.169µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.117µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.317µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.64µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.033µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.964µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.15µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.259µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.665µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.175µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.314µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 132.38µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.638µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.865µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.809µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.377µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 404.412µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.498µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.058µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.397µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.546µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.125µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.172µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317928, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.631µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.877µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.768µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.2µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.036µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.849µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.058µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.146µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.79µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.513µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.041µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.023µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.443µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.184µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.649µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.083µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.6516985, 0.06134387, 11.157986],
     [-0.6545333, 0.753722, 0.059068833, 7.207299],
     [-0.007741158, -0.08480724, 0.9963671, -0.09297203],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with Front-Back_Flip_Reverse-X:
    [[-1.0, 0.0, 0.0, -6.7348967],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 125.597µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 126.308µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.041µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 863.691µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 89.85µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.243µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 405.154µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.353µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.485µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.974µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.784µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.07µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.399µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.79µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 105.178µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.164µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.526µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.862µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.858µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.976µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.934µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.999µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.106µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.194µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.985µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.374µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.018µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.604µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.684µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.453µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.814µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.143µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.254µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.095µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.63µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.114µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.585µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.841µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.992µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.208µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.684µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.584µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.066µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 133.023µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.493µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.084µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.232µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.052µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.894µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.43µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.181µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.629µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.626µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.077µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.799µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.524µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.657µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.167µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.756µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.632µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.255µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.595µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.249µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.088µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.889µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559936, 0.6516985, 0.061343856, 11.157986],
     [-0.6545333, 0.7537221, 0.05906882, 7.207299],
     [-0.007741159, -0.08480724, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with Rot_180_Z:
    [[-1.0, 0.0, 0.0, -6.7348967],
     [0.0, -1.0, 0.0, -9.637826],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 124.214µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 123.473µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.073µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 861.717µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 90.381µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.083µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.704µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.858µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.328µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.586µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.845µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.05µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 413.15µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 136.767µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 109.507µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 406.738µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.775µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.5µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.474µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 115.688µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 88.418µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.5µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.59µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 88.207µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 384.507µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 115.819µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.037µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.539µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 134.394µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.383µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.149µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.263µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.824µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.911µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 127.392µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.113µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.824µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.503µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.206µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.595µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.304µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.423µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.905µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.884µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.834µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.948µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.583µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.125µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 412.419µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 137.89µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 103.015µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.069µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.059µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.193µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.397µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.445µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.886µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.358µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.878µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.104µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.646µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.677µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.16µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.294µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.784µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559937, 0.6516984, 0.061343867, 11.157986],
     [-0.6545332, 0.75372195, 0.059068844, 7.2072988],
     [-0.0077411574, -0.08480724, 0.9963671, -0.09297205],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan] Combined initial transformation with Rot_180_Y:
    [[-1.0, 0.0, 0.0, -6.7348967],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, -1.0, -0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 130.357µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 120.367µs
[2026-03-03T12:37:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.392µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 864.502µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 88.197µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.823µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.004µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.413µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.567µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 407.308µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.862µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.374µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.036µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.455µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.092µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.452µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.946µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.417µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.164µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.483µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.954µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.597µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.105µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.88µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.417µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 130.627µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.102µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.751µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.416µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.733µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 402.64µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.257µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.524µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.584µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.153µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.443µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.157µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.005µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 104.107µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.616µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.725µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.675µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.608µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.822µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.612µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.716µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.627µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.485µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.366µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.556µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.306µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.136µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.908µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.896µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.323µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.775µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.051µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.69µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.975µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.564µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.115µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 131.578µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.954µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.789µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.964µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.65169847, 0.06134385, 11.157986],
     [-0.6545334, 0.7537221, 0.05906881, 7.207299],
     [-0.00774116, -0.08480723, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_180_X:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, -1.0, 0.0, -9.637826],
     [0.0, 0.0, -1.0, -0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 136.698µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 131.719µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.198µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 867.337µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 89.137µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 106.371µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.645µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 128.963µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 104.558µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.598µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.316µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.046µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.086µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.842µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.063µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.054µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 130.237µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.396µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 403.201µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 155.665µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.661µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.944µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.513µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.903µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.011µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 115.068µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.319µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.364µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.159µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.004µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.34µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.149µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 88.828µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.418µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.54µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.698µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.385µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.029µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.255µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.88µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.382µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.055µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.732µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.951µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.699µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.601µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 129.565µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.79µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.357µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 115.989µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 88.959µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 404.163µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 131.809µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.955µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.477µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.514µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.958µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.797µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.23µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.681µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.823µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.443µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.712µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.812µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.902µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.65169847, 0.061343852, 11.157986],
     [-0.65453327, 0.753722, 0.05906882, 7.2072988],
     [-0.007741159, -0.08480724, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_90_X:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, 0.0, -1.0, -0.27938053],
     [0.0, 1.0, 0.0, 9.637826],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 125.096µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 119.896µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.472µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 869.533µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 89.349µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.39µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.476µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.164µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.954µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.083µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.253µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.115µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.165µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.85µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.502µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.024µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.897µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.943µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.649µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.764µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.475µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.989µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 129.095µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.588µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.564µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.984µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.193µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.363µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 116.189µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.253µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.491µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.714µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.681µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.08µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.634µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.986µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.358µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.19µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.393µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.225µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.565µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.714µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.594µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.325µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.084µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.646µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318211, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.584µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.262µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.104µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.504µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.436µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.391µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.473µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.938µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.798µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.233µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.155µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 404.685µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.974µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.134µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.369µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.984µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.641µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.501µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.963µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559937, 0.6516984, 0.06134385, 11.157986],
     [-0.6545333, 0.753722, 0.059068825, 7.2072988],
     [-0.0077411598, -0.08480723, 0.9963671, -0.09297204],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_-90_X:
    [[1.0, 0.0, 0.0, 6.7348967],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, -1.0, 0.0, -9.637826],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 125.977µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 121.048µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.504µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 861.126µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 89.849µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.413µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.406µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.717µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.825µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.307µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.965µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.742µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.285µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.734µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.383µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 387.963µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.717µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.964µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.207µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.865µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.678µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.436µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 119.095µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.362µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.221µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.444µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.209µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.464µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 118.574µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.692µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 386.94µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.883µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 89.969µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.628µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 117.863µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 91.682µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.146144ms
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.007µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.589µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 390.207µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.718µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.134µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 388.774µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.627µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.926µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 416.164µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 137.77µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 110.138µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 409.583µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 137.349µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 103.425µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.772µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.547µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 92.765µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 389.325µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.21µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.278µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.463µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.346µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.897µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.574µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.01µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.961µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 402.369µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 136.758µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.6516985, 0.061343864, 11.157986],
     [-0.6545333, 0.753722, 0.05906884, 7.207299],
     [-0.0077411584, -0.08480723, 0.9963671, -0.092972055],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_90_Y:
    [[0.0, 0.0, 1.0, 0.27938053],
     [0.0, 1.0, 0.0, 9.637826],
     [-1.0, 0.0, 0.0, -6.7348967],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 131.999µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 126.078µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 108.555µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 867.909µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 93.447µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.052µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.637µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 124.385µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.512µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 399.004µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.844µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 101.132µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.644µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.859µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 113.004µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.122µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.534µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.93µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.066µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.882µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.441µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.133µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.978µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.591µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.523µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.484µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.49µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 405.985µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.043µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.911µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 391.969µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.882µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.14µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.312µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.557µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.831µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.891µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 130.797µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.408µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 410.765µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 127.591µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.376µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.509µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 127.28µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.895µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 406.928µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.748µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.557µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.051µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.707µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.877µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 399.345µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.838µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.027µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 410.596µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.988µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.716µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.263µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.256µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.594µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.752µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.378µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.947µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.969µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.358µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.65169847, 0.06134386, 11.157986],
     [-0.65453327, 0.753722, 0.059068833, 7.2072988],
     [-0.007741159, -0.08480724, 0.9963671, -0.09297204],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_-90_Y:
    [[0.0, 0.0, -1.0, -0.27938053],
     [0.0, 1.0, 0.0, 9.637826],
     [1.0, 0.0, 0.0, 6.7348967],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 130.977µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 133.181µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.779µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 869.141µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 96.363µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.537µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.149µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.35µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.564µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.389µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 131.329µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.677µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 402.231µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.487µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.745µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.89µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 130.336µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 114.366µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.88µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.227µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.715µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.521µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.978µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.356µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.493µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 141.677µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.608µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.83µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.988µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 106.13µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.387µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 135.656µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.802µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 398.623µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.557µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 98.386µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.642µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.727µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.284µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.079µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.698µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.694µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 401.488µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 129.494µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.749µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.351µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.432µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.417µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.556µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.261µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.151µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.272µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.443µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.112µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.64µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.77µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.316µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 129.404µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.759µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.824µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.737µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.659µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 399.935µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.551µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.65169847, 0.06134385, 11.157986],
     [-0.65453327, 0.753722, 0.059068825, 7.2072988],
     [-0.007741159, -0.08480724, 0.9963671, -0.09297205],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_90_Z:
    [[0.0, -1.0, 0.0, -9.637826],
     [1.0, 0.0, 0.0, 6.7348967],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 127.451µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 125.487µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.773µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 865.645µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 93.818µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.639µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.59µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.539µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.677µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.731µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707486, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.14µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.029µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.182µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.52µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.509µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.382µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.031µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.199µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 400.927µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.211µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 101.061µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.868µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.122µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.559µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.355µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.008µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.409µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 405.627µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.902µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.64µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.504µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.011µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.943µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.175µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.357µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.309µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.714µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.03µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.172µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.048µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.199µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 107.012µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.332µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.409µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.27µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.684µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.439µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 102.252µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.786µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 139.973µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.991µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.425µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.7µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.098µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.439µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 125.656µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.131µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.12µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 120.929µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.166µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.932µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.131µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 107.693µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.161µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.574µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7559936, 0.65169835, 0.061343838, 11.157985],
     [-0.65453327, 0.753722, 0.059068818, 7.2072988],
     [-0.00774116, -0.08480724, 0.9963671, -0.09297203],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Combined initial transformation with Rot_-90_Z:
    [[0.0, 1.0, 0.0, 9.637826],
     [-1.0, 0.0, 0.0, -6.7348967],
     [0.0, 0.0, 1.0, 0.27938053],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 128.142µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 124.746µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.198µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 865.794µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 94.92µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Starting ICP iterations...
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 1 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.573µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.667µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 0: RMSE = 0.735016, Valid points = 5697
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.241µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 2 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.21µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 402.27µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 1: RMSE = 0.707487, Valid points = 5727
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.81µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 3 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 103.295µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 396.92µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 2: RMSE = 0.640419, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.71µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 4 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.448µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 399.936µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 3: RMSE = 0.553956, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.904µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 5 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.452µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.333µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 4: RMSE = 0.488720, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.212µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 6 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.669µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 392.922µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 5: RMSE = 0.440125, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.929µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 7 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.279µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.978µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 6: RMSE = 0.410457, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.703µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 8 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.627µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 397.681µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 7: RMSE = 0.393512, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.118µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 9 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.389µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.895µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 8: RMSE = 0.383819, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 126.739µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 10 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 105.369µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.604µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 9: RMSE = 0.377578, Valid points = 5738
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.9µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 11 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.019µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.015µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 10: RMSE = 0.326075, Valid points = 5579
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.4µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 12 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.283µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 393.814µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 11: RMSE = 0.320725, Valid points = 5575
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 138.661µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 13 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 94.088µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.193µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 12: RMSE = 0.319864, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.05µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 14 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 103.265µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.457µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 13: RMSE = 0.318212, Valid points = 5581
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.368µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 15 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.831µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.316µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 14: RMSE = 0.317929, Valid points = 5582
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.332µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 16 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 93.677µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.284µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 15: RMSE = 0.220168, Valid points = 4908
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.831µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 17 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.402µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 394.734µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 16: RMSE = 0.213434, Valid points = 4886
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.422µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 18 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.082µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 395.236µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 17: RMSE = 0.211538, Valid points = 4872
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 121.89µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 19 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.651µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 409.183µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 18: RMSE = 0.210819, Valid points = 4867
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 123.644µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] --- ICP Iteration 20 ---
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 95.21µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 400.427µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::registration] Iter 19: RMSE = 0.210749, Valid points = 4868
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.351µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.75599366, 0.65169847, 0.06134385, 11.157986],
     [-0.65453327, 0.753722, 0.059068825, 7.2072988],
     [-0.007741159, -0.08480724, 0.9963671, -0.09297203],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 97.064µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Original
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 99.177µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_LR_Flip_Reverse-Y_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: LR_Flip_Reverse-Y
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 107.072µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_UD_Flip_Reverse-Z_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: UD_Flip_Reverse-Z
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.532µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Front-Back_Flip_Reverse-X_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Front-Back_Flip_Reverse-X
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 104.888µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Z_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Z
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 114.136µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Y_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Y
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 120.809µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_X_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_180_X
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 117.782µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_X_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_90_X
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 111.871µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_X_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_X
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 154.161µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Y_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Y
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 127.692µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Y
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 112.322µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Z
[2026-03-03T12:37:07Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 125.718µs
[2026-03-03T12:37:07Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Z_iter-20.pcd
[2026-03-03T12:37:07Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Z

```