# 02/03/2026
```bash
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] === Available Vulkan Devices ===
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Device 0: NVIDIA GeForce RTX 4080 (DiscreteGpu)
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Device 1: llvmpipe (LLVM 15.0.7, 256 bits) (Cpu)
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Selected device: NVIDIA GeForce RTX 4080 (DiscreteGpu)
    
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] === Vulkan Device Information ===
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Device Name: NVIDIA GeForce RTX 4080
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Device Type: DiscreteGpu
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] Vulkan context initialized successfully.
[2026-03-02T12:42:28Z DEBUG registration_vulkan::init_gpu] ===============================
    
[2026-03-02T12:42:28Z INFO  registration_vulkan] === Parameters ===
[2026-03-02T12:42:28Z INFO  registration_vulkan] Source PCD path: data/input/H927/lab-room_voxel_025_xyz_only.pcd
[2026-03-02T12:42:28Z INFO  registration_vulkan] Target PCD path: data/input/H927/lab-room_voxel_025_xyz_only.pcd
[2026-03-02T12:42:28Z INFO  registration_vulkan] Voxel size: 0.25
[2026-03-02T12:42:28Z INFO  registration_vulkan] ====================
[2026-03-02T12:42:28Z INFO  registration_vulkan] Points num of source: 7285
[2026-03-02T12:42:28Z INFO  registration_vulkan] Points num of target: 7285
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 7285 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 447.153µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5864
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Reallocating buffers for 7285 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Compute voxelization shader execution time: 128.571µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_voxel] Number of output points: 5864
[2026-03-02T12:42:28Z DEBUG registration_vulkan] Downsampled points num of source: 5864
[2026-03-02T12:42:28Z DEBUG registration_vulkan] Downsampled points num of target: 5864
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 132µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_covariance] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_covariance] Compute covariance shader execution time: 129.355µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_transform] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 96.272µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan] Saving downsampled source points with covariances to: data/output/debug/downsampled_source_with_covs.pcd
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_search_neighbor] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 404.913µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_knn_search] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_knn_search] Compute knn search shader execution time: 866.543µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_normals] Reallocating buffers for 5864 points
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_normals] Compute normals shader execution time: 93.566µs
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_icp] Allocating buffers for H and b matrix
[2026-03-02T12:42:28Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 122.059µs
H (6x6 matrix):
61510.609375 -60291.722656 3861.401123 -102.405838 551.479370 12058.958984
-60291.730469 145126.468750 1940.336304 -445.494141 79.714241 -18205.761719
3861.401123 1940.336304 76699.218750 -2751.077637 7684.389160 22.691618
-102.405853 -445.494141 -2751.077637 794.995728 -38.151367 -22.237442
551.479370 79.714233 7684.388672 -38.151367 1343.317261 -6.154661
12058.959961 -18205.761719 22.691610 -22.237442 -6.154661 3725.686768
b (6x1 vector):
0.000000
0.000000
0.000000
0.000000
0.000000
0.000000
Delta transformation matrix:
1.000000 -0.000000 0.000000 0.000000
0.000000 1.000000 -0.000000 0.000000
-0.000000 0.000000 1.000000 0.000000
0.000000 0.000000 0.000000 1.000000
[2026-03-02T12:42:28Z DEBUG registration_vulkan] RMSE: 0, count of valid correspondences: 5864
```