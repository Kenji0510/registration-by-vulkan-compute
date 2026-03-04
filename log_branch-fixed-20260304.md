# 04/03/2026
## RTX 4080
### H927 data
```bash
2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 39.294µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 77.456µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.409232, Valid points = 1765
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 72.107µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 39.345µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 82.534µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.409136, Valid points = 1765
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 67.958µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 39.083µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 79.931µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.409100, Valid points = 1765
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.439µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 38.191µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 76.925µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.409043, Valid points = 1765
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.048µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 39.344µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 76.655µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.408999, Valid points = 1765
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 70.995µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7345347, 0.6753639, 0.06589615, 11.236619],
     [-0.6785062, 0.7323367, 0.05755214, 7.1336365],
     [-0.009389555, -0.08698504, 0.9961653, -0.2028863],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan] Final RMSE: 0.40899855
[2026-03-04T05:33:20Z INFO  registration_vulkan] Total registration time: 223.58ms
[2026-03-04T05:33:20Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 45.796µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Z
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Z: 0.37512222
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Z:
    [[-0.74399906, -0.6621918, -0.089257896, -1.1306822],
     [0.6678921, -0.74093485, -0.07025175, -0.680517],
     [-0.019614194, -0.11188192, 0.9935279, -0.44810715],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 47.832µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Z_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Z
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Z: 0.37546733
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Z:
    [[-0.74811774, -0.65752137, -0.08936098, -1.0891786],
     [0.66327566, -0.7449608, -0.07140832, -0.69936043],
     [-0.019617926, -0.112692855, 0.99343634, -0.450637],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 67.347µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Front-Back_Flip_Reverse-X_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Front-Back_Flip_Reverse-X
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Front-Back_Flip_Reverse-X: 0.39689338
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Front-Back_Flip_Reverse-X:
    [[-0.73889124, -0.66756594, -0.09162567, -1.116791],
     [-0.6730707, 0.73764, 0.053505912, 6.9700007],
     [-0.031868063, -0.10120559, 0.9943551, -0.39494276],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 69.131µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Z_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Z
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Z: 0.40899855
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Z:
    [[0.7345347, 0.6753639, 0.06589615, 11.236619],
     [-0.6785062, 0.7323367, 0.05755214, 7.1336365],
     [-0.009389555, -0.08698504, 0.9961653, -0.2028863],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 70.162µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Original
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Original: 0.40930834
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Original:
    [[0.7509362, 0.6578563, 0.057617877, 11.132158],
     [-0.6603609, 0.748619, 0.059101313, 7.221912],
     [-0.0042536655, -0.08242989, 0.99658805, -0.17862934],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 82.766µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_X_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_X
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_X: 0.40930843
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_X:
    [[0.7509365, 0.6578561, 0.057617787, 11.132156],
     [-0.66036093, 0.74861896, 0.059101272, 7.221912],
     [-0.004253663, -0.082429856, 0.99658763, -0.17862938],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 73.699µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Y_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Y
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Y: 0.41118008
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Y:
    [[-0.7380369, -0.66928226, -0.08580826, -1.098859],
     [-0.67434984, 0.73603547, 0.05919643, 6.9511757],
     [0.023538813, 0.10155397, -0.9945514, -0.16087882],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 67.758µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Y
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Y: 0.41410726
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Y:
    [[-0.752542, -0.6528997, -0.086033136, -1.008392],
     [-0.6580789, 0.75047386, 0.060992073, 7.035703],
     [0.024743924, 0.102515735, -0.99442333, -0.15631178],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 72.277µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_LR_Flip_Reverse-Y_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: LR_Flip_Reverse-Y
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation LR_Flip_Reverse-Y: 0.41686076
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for LR_Flip_Reverse-Y:
    [[0.6812712, 0.7293492, 0.06260755, 11.385648],
     [0.73202413, -0.67838764, -0.06269909, -0.046903674],
     [0.0032573687, -0.08854531, 0.99606687, -0.21709727],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 90.23µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Y_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Y
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Y: 0.41888595
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Y:
    [[0.7805055, 0.6227341, 0.054895084, 10.761727],
     [-0.62513196, 0.77811265, 0.061238978, 7.487277],
     [-0.0045789345, -0.082114086, 0.99661225, -0.18157397],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.824µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_UD_Flip_Reverse-Z_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: UD_Flip_Reverse-Z
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation UD_Flip_Reverse-Z: 0.43179947
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for UD_Flip_Reverse-Z:
    [[0.7531637, 0.65392303, 0.071617536, 11.081628],
     [-0.6577688, 0.7501356, 0.06809177, 7.2248316],
     [0.0091961175, 0.09839202, -0.9951051, -0.20918274],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 59.373µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_X_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_90_X
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_X: 0.4376645
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_X:
    [[0.6859461, 0.7233582, 0.078938864, 11.357416],
     [0.72755784, -0.6835601, -0.05835605, -0.11145366],
     [0.011747149, 0.09746157, -0.9951698, -0.2093859],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 66.505µs
[2026-03-04T05:33:20Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_X_iter-40.pcd
[2026-03-04T05:33:20Z INFO  registration_vulkan] Saved results for transformation: Rot_180_X
[2026-03-04T05:33:20Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_X: 0.43766454
[2026-03-04T05:33:20Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_X:
    [[0.6859458, 0.723358, 0.07893879, 11.357414],
     [0.7275576, -0.6835599, -0.058356006, -0.11145288],
     [0.011747136, 0.097461596, -0.99517024, -0.20938608],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:33:20Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:33:20Z INFO  registration_vulkan] Best registration result: Rot_90_Z
[2026-03-04T05:33:20Z INFO  registration_vulkan] Best RMSE: 0.37512222
[2026-03-04T05:33:20Z INFO  registration_vulkan] Best ICP transformation matrix:
    [[-0.74399906, -0.6621918, -0.089257896, -1.1306822],
     [0.6678921, -0.74093485, -0.07025175, -0.680517],
     [-0.019614194, -0.11188192, 0.9935279, -0.44810715],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
```

### Rentallab
```bash

```