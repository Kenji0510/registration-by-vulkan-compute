# 04/03/2026
## RTX 4080
```bash
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 35 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.454µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 324.372µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 34: RMSE = 0.364827, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 72.918µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 59.072µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 329.312µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.364812, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 73.999µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 53.05µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 324.123µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.364802, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 72.857µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 53.031µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 336.606µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.364797, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 76.665µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 53.372µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 325.233µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.364798, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 72.126µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 52.928µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 324.202µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.364806, Valid points = 5738
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 73.909µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.74636126, 0.66306597, 0.057345457, 11.225869],
     [-0.665483, 0.7446601, 0.051126614, 7.203365],
     [-0.008802547, -0.07632133, 0.9970444, -0.10658069],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T16:16:33Z DEBUG registration_vulkan] Final RMSE: 0.3648059
[2026-03-03T16:16:33Z INFO  registration_vulkan] Total registration time: 387.18ms
[2026-03-03T16:16:33Z INFO  registration_vulkan] === Registration Results ===
[2026-03-03T16:16:33Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 54.472µs
[2026-03-03T16:16:33Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
```

## M4 pro (on Mac OS)
```bash
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 35 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 205.916µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.272041ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 34: RMSE = 0.364827, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 397.167µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 209.458µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.134625ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.364812, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 393.792µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 243.959µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.206542ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.364802, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 357.375µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 220.667µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.253ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.364796, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 401.209µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 391.042µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.175542ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.364798, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 351.583µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 204.625µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 1.175083ms
[2026-03-03T15:52:21Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.364806, Valid points = 5738
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 323.958µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7463611, 0.6630661, 0.057345472, 11.225869],
     [-0.66548306, 0.7446601, 0.051126614, 7.2033653],
     [-0.008802554, -0.07632133, 0.9970444, -0.10658066],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T15:52:21Z DEBUG registration_vulkan] Final RMSE: 0.36480585
[2026-03-03T15:52:21Z INFO  registration_vulkan] Total registration time: 1.09s
[2026-03-03T15:52:21Z INFO  registration_vulkan] === Registration Results ===
[2026-03-03T15:52:21Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 284.083µs
[2026-03-03T15:52:21Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
Error: Failed to save results
```

## RTX 3050
```bash
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.364811, Valid points = 5738
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 72.004µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 61.426µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 462.008µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.364802, Valid points = 5738
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 70.558µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.724µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 461.034µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.364797, Valid points = 5738
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 69.269µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.467µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 476.011µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.364798, Valid points = 5738
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 70.116µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.512µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 475.111µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.364806, Valid points = 5738
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 71.262µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7463613, 0.663066, 0.05734548, 11.225869],
     [-0.6654832, 0.74466014, 0.051126625, 7.2033653],
     [-0.008802544, -0.07632133, 0.9970444, -0.10658071],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-03T16:12:06Z DEBUG registration_vulkan] Final RMSE: 0.36480582
[2026-03-03T16:12:06Z INFO  registration_vulkan] Total registration time: 512.33ms
[2026-03-03T16:12:06Z INFO  registration_vulkan] === Registration Results ===
[2026-03-03T16:12:06Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 61.038µs
[2026-03-03T16:12:06Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
Error: Failed to save results

Caused by:
    0: Failed to save aligned source and target point cloud
    1: No such file or directory (os error 2)
```