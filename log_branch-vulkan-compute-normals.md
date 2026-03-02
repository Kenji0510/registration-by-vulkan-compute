# 02/03/2026
```bash
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] === Available Vulkan Devices ===
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Device 0: NVIDIA GeForce RTX 4080 (DiscreteGpu)
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Selected device: NVIDIA GeForce RTX 4080 (DiscreteGpu)
    
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] === Vulkan Device Information ===
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Device Name: NVIDIA GeForce RTX 4080
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Device Type: DiscreteGpu
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] Vulkan context initialized successfully.
[2026-03-02T06:51:39Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-02T06:51:39Z INFO  registration_vulkan] === Parameters ===
[2026-03-02T06:51:39Z INFO  registration_vulkan] Source PCD path: data/input/H927/lab-room_voxel_025_xyz_only.pcd
[2026-03-02T06:51:39Z INFO  registration_vulkan] Target PCD path: data/input/H927/lab-room_voxel_025_xyz_only.pcd
[2026-03-02T06:51:39Z INFO  registration_vulkan] Voxel size: 0.25
[2026-03-02T06:51:39Z INFO  registration_vulkan] ====================
[2026-03-02T06:51:39Z INFO  registration_vulkan] Points num of source: 7285
[2026-03-02T06:51:39Z INFO  registration_vulkan] Points num of target: 7285
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 7285 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 413.401µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5864
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 7285 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 85.21µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5864
[2026-03-02T06:51:39Z DEBUG registration_vulkan] Downsampled points num of source: 5864
[2026-03-02T06:51:39Z DEBUG registration_vulkan] Downsampled points num of target: 5864
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 111.59µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 94.749µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_transform] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 64.3µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan] Saving downsampled source points with covariances to: data/output/debug/downsampled_source_with_covs.pcd
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_search_neighbor] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 364.077µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_knn_search] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 845.537µs
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_normals] Reallocating buffers for 5864 points
[2026-03-02T06:51:39Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 67.177µs
```