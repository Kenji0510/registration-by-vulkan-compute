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
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 35 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 40.156µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 80.632µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 34: RMSE = 0.362233, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 67.948µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 41.759µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 84.499µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.362233, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 69.801µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 43.342µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 82.013µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.362233, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 70.763µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 45.596µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 83.066µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.362232, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.959µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 43.813µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 84.378µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.362233, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 71.664µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 45.056µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 82.596µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.362232, Valid points = 882
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 69.581µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan] Final transformation matrix:
    [[-0.23554933, 0.7947544, -0.5593589, 3.486178],
     [-0.96203727, -0.27231014, 0.018212948, -0.441025],
     [-0.13784423, 0.5424142, 0.82872564, 2.0598104],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan] Final RMSE: 0.36223245
[2026-03-04T05:37:38Z INFO  registration_vulkan] Total registration time: 224.98ms
[2026-03-04T05:37:38Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 45.806µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Z_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Z
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Z: 0.23731095
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Z:
    [[-0.8999838, -0.26830843, -0.34356844, -1.738331],
     [0.24782959, -0.9633028, 0.10309325, -1.828406],
     [-0.35862124, 0.0076358505, 0.93345195, -0.3389655],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 60.525µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_X_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_180_X
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_X: 0.3125789
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_X:
    [[0.7673601, 0.53573537, 0.3523444, 2.5526123],
     [0.49865314, -0.84403723, 0.19734678, -1.1814896],
     [0.40311712, 0.02426159, -0.91482663, 2.7444613],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 111.881µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_X_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_X
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_X: 0.31257892
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_X:
    [[0.7673601, 0.5357353, 0.35234413, 2.552612],
     [0.4986533, -0.8440374, 0.19734664, -1.18149],
     [0.40311715, 0.024261644, -0.9148264, 2.7444615],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 56.106µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_LR_Flip_Reverse-Y_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: LR_Flip_Reverse-Y
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation LR_Flip_Reverse-Y: 0.31811878
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for LR_Flip_Reverse-Y:
    [[0.77878356, 0.37799215, -0.50061744, 2.945054],
     [0.51026213, -0.84592247, 0.15507242, -1.1272672],
     [0.36486736, 0.37621439, 0.851666, 1.9020145],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 49.954µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_UD_Flip_Reverse-Z_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: UD_Flip_Reverse-Z
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation UD_Flip_Reverse-Z: 0.32499442
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for UD_Flip_Reverse-Z:
    [[0.88225025, 0.2559026, 0.39515594, 1.4668307],
     [-0.21404278, 0.9656303, -0.14745586, 5.620389],
     [0.419309, -0.04551266, -0.9067022, 2.4685512],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.187µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Y_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Y
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Y: 0.33074072
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Y:
    [[-0.7510642, -0.3695085, 0.5471429, -3.0194914],
     [-0.5132707, 0.8480379, -0.13185292, 4.74121],
     [-0.41527712, -0.37986282, -0.8265885, 0.0987267],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 47.1µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Y_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Y
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Y: 0.33634347
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Y:
    [[-0.7686788, -0.38715857, 0.50915676, -3.0324848],
     [-0.53550225, 0.8248573, -0.18123825, 4.7712083],
     [-0.34981367, -0.41196877, -0.8413753, 0.060688186],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.246µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Front-Back_Flip_Reverse-X_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Front-Back_Flip_Reverse-X
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Front-Back_Flip_Reverse-X: 0.34857926
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Front-Back_Flip_Reverse-X:
    [[-0.79966414, -0.49740866, -0.33633718, -2.3891535],
     [-0.4568223, 0.8675086, -0.19683151, 4.8725452],
     [-0.38968062, 0.0037528034, 0.9209421, -0.449092],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.196µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Z
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Z: 0.35715428
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Z:
    [[0.16814664, -0.9581559, -0.23165418, -3.2827554],
     [0.97844636, 0.19080214, -0.07897892, 3.6927407],
     [0.119874194, -0.21338113, 0.96958673, -0.8162627],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 48.433µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Original
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Original: 0.3613042
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Original:
    [[0.9024485, 0.05881896, -0.42676416, 1.6890552],
     [-0.1768169, 0.95392054, -0.24242757, 5.644493],
     [0.3928399, 0.2942374, 0.8712641, 1.6065733],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.468µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Y
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Y: 0.3614394
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Y:
    [[0.89133114, 0.07631361, -0.44688365, 1.7840372],
     [-0.1972368, 0.95282865, -0.2306853, 5.5992966],
     [0.40819907, 0.29375896, 0.8643373, 1.6342697],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 50.786µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Z_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Z
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Z: 0.36223245
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Z:
    [[-0.23554933, 0.7947544, -0.5593589, 3.486178],
     [-0.96203727, -0.27231014, 0.018212948, -0.441025],
     [-0.13784423, 0.5424142, 0.82872564, 2.0598104],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 47.66µs
[2026-03-04T05:37:38Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_X_iter-40.pcd
[2026-03-04T05:37:38Z INFO  registration_vulkan] Saved results for transformation: Rot_90_X
[2026-03-04T05:37:38Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_X: 0.36358622
[2026-03-04T05:37:38Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_X:
    [[0.877069, 0.11888618, -0.4654206, 2.0010912],
     [-0.25552303, 0.9359072, -0.24245772, 5.4940605],
     [0.40676582, 0.33157784, 0.85123295, 1.7993176],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:37:38Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:37:38Z INFO  registration_vulkan] Best registration result: Rot_180_Z
[2026-03-04T05:37:38Z INFO  registration_vulkan] Best RMSE: 0.23731095
[2026-03-04T05:37:38Z INFO  registration_vulkan] Best ICP transformation matrix:
    [[-0.8999838, -0.26830843, -0.34356844, -1.738331],
     [0.24782959, -0.9633028, 0.10309325, -1.828406],
     [-0.35862124, 0.0076358505, 0.93345195, -0.3389655],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
```

### AIST
```bash
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 35 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 36.509µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 85.572µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 34: RMSE = 0.336768, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 65.043µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 36.509µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 85.822µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.336783, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 64.683µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 36.639µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 86.023µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.336800, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 64.863µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 37.07µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 85.572µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.336801, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 66.325µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 37.131µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 85.491µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.336809, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 65.734µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 37.672µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 85.972µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.336810, Valid points = 1527
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 65.643µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7775821, 0.026609141, -0.6282181, -0.22230698],
     [-0.5633373, 0.47330073, -0.67722803, 3.7185214],
     [0.27931577, 0.8804989, 0.38301995, 5.5262036],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan] Final RMSE: 0.33680964
[2026-03-04T05:40:35Z INFO  registration_vulkan] Total registration time: 265.77ms
[2026-03-04T05:40:35Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 37.612µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Original
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Original: 0.33675376
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Original:
    [[0.7773476, 0.026430305, -0.6285156, -0.224589],
     [-0.56342405, 0.4736301, -0.6769254, 3.7207181],
     [0.27979264, 0.8803269, 0.38306692, 5.5226526],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 53.45µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_X_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_90_X
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_X: 0.33675945
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_X:
    [[0.777505, 0.026528053, -0.6283168, -0.22465128],
     [-0.56341994, 0.47321475, -0.6772194, 3.7183673],
     [0.27936342, 0.8805474, 0.3828733, 5.525546],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 53.371µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Z_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Z
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Z: 0.33680964
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Z:
    [[0.7775821, 0.026609141, -0.6282181, -0.22230698],
     [-0.5633373, 0.47330073, -0.67722803, 3.7185214],
     [0.27931577, 0.8804989, 0.38301995, 5.5262036],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 56.517µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_UD_Flip_Reverse-Z_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: UD_Flip_Reverse-Z
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation UD_Flip_Reverse-Z: 0.38973275
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for UD_Flip_Reverse-Z:
    [[0.73215544, 0.17506006, 0.65825707, 0.30664286],
     [-0.61530185, 0.5844927, 0.5289348, 4.0721045],
     [0.29215115, 0.7922891, -0.5356542, 5.140943],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 56.798µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Z
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Z: 0.4233405
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Z:
    [[-0.73568475, -0.1761905, -0.6540066, -0.96482044],
     [0.6258629, -0.54600626, -0.556931, -2.5178733],
     [-0.2589657, -0.8190447, 0.51195985, -4.398041],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 49.824µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Z_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Z
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Z: 0.42348474
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Z:
    [[-0.7380315, -0.17453468, -0.65180236, -0.96651936],
     [0.62300116, -0.5472983, -0.5588685, -2.5255637],
     [-0.2591885, -0.81853664, 0.5126587, -4.3990507],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.326µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Front-Back_Flip_Reverse-X_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Front-Back_Flip_Reverse-X
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Front-Back_Flip_Reverse-X: 0.4455383
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Front-Back_Flip_Reverse-X:
    [[0.40538135, -0.67656314, -0.6147584, -4.077231],
     [-0.7943903, 0.07204469, -0.60312045, 1.2587218],
     [-0.4523391, -0.73285186, 0.50824964, -3.910016],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 45.276µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_X_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_X
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_X: 0.4467503
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_X:
    [[0.45730302, -0.559678, 0.6911106, -4.0194283],
     [-0.8288962, 0.013298456, 0.5592441, 0.78063667],
     [-0.32218745, -0.8286032, -0.45783362, -4.2441993],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 49.122µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Y_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_90_Y
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_90_Y: 0.45116964
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_90_Y:
    [[0.4366694, -0.57110226, 0.69509816, -4.011809],
     [-0.8268294, 0.04969483, 0.5602539, 0.95498776],
     [-0.35450524, -0.81937313, -0.45050415, -4.1351066],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 63.992µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_X_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_180_X
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_X: 0.45659298
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_X:
    [[-0.28653082, 0.7445088, 0.6029977, 4.0924087],
     [0.75884247, -0.2078526, 0.617216, -1.1719576],
     [0.58485734, 0.63443184, -0.5054088, 4.1096473],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 67.647µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_180_Y_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_180_Y
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_180_Y: 0.53427434
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_180_Y:
    [[0.16006136, -0.65540206, 0.73812497, -4.075901],
     [-0.79448557, 0.35823324, 0.4903687, 2.4287524],
     [-0.58580977, -0.6649186, -0.46336806, -2.9182823],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 66.054µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_-90_Y_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: Rot_-90_Y
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation Rot_-90_Y: 0.62226653
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for Rot_-90_Y:
    [[0.61582136, -0.5203553, -0.5916026, -3.0780046],
     [-0.03265871, 0.73336965, -0.67904544, 4.3917007],
     [0.7872085, 0.43749166, 0.43463063, 2.3544586],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 56.106µs
[2026-03-04T05:40:35Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_LR_Flip_Reverse-Y_iter-40.pcd
[2026-03-04T05:40:35Z INFO  registration_vulkan] Saved results for transformation: LR_Flip_Reverse-Y
[2026-03-04T05:40:35Z INFO  registration_vulkan] Final RMSE for transformation LR_Flip_Reverse-Y: 0.75048906
[2026-03-04T05:40:35Z INFO  registration_vulkan] ICP transformation matrix for LR_Flip_Reverse-Y:
    [[0.53584087, 0.39723676, -0.7450351, 2.9998033],
     [0.03930968, -0.8931901, -0.44795743, -4.4698277],
     [0.8434037, -0.21074675, 0.49422303, 0.48430443],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:40:35Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:40:35Z INFO  registration_vulkan] Best registration result: Original
[2026-03-04T05:40:35Z INFO  registration_vulkan] Best RMSE: 0.33675376
[2026-03-04T05:40:35Z INFO  registration_vulkan] Best ICP transformation matrix:
    [[0.7773476, 0.026430305, -0.6285156, -0.224589],
     [-0.56342405, 0.4736301, -0.6769254, 3.7207181],
     [0.27979264, 0.8803269, 0.38306692, 5.5226526],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
```


# RTX 3050
## AIST
```bash
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 37.837µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 119.175µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.336783, Valid points = 1527
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 74.683µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 35.749µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 118.602µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.336800, Valid points = 1527
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 69.279µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 33.651µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 120.244µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.336801, Valid points = 1527
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.826µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 33.687µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 119.666µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.336809, Valid points = 1527
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.547µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 34.814µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 117.073µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.336810, Valid points = 1527
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 68.756µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7775821, 0.026609126, -0.62821835, -0.22230706],
     [-0.5633374, 0.4733007, -0.677228, 3.718521],
     [0.2793158, 0.8804989, 0.38301995, 5.5262036],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:45:26Z DEBUG registration_vulkan] Final RMSE: 0.33680972
[2026-03-04T05:45:26Z INFO  registration_vulkan] Total registration time: 297.49ms
[2026-03-04T05:45:26Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:45:26Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 33.299µs
[2026-03-04T05:45:26Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-40.pcd
Error: Failed to save results

Caused by:
    0: Failed to save aligned source and target point cloud
    1: No such file or directory (os error 2)
```

# M4 pro (on Mac OS)
```bash
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 116.167µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 560.125µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.336783, Valid points = 1527
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 237.917µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 114.75µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 514.959µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.336800, Valid points = 1527
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 228.167µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 107.042µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 509.916µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.336800, Valid points = 1527
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 235.875µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 110.209µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 510.375µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.336809, Valid points = 1527
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 229.542µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 132.709µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 514.791µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.336810, Valid points = 1527
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 220.417µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7775822, 0.026609087, -0.6282183, -0.22230737],
     [-0.56333745, 0.47330076, -0.6772282, 3.7185216],
     [0.2793158, 0.8804989, 0.38301995, 5.5262036],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T05:48:58Z DEBUG registration_vulkan] Final RMSE: 0.33680975
[2026-03-04T05:48:58Z INFO  registration_vulkan] Total registration time: 577.81ms
[2026-03-04T05:48:58Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T05:48:58Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 165.125µs
[2026-03-04T05:48:58Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Original_iter-40.pcd
```

# M4 pro (on Fedora VM)
```bash
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.409232, Valid points = 1765
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 294.416µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 295.625µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 293.166µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.409136, Valid points = 1765
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 309.417µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 292.333µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 297.083µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.409100, Valid points = 1765
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 308.5µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 283.166µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 294.708µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.409043, Valid points = 1765
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 305.999µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 319.583µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 292.458µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.408998, Valid points = 1765
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 346.75µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.7345347, 0.6753642, 0.06589618, 11.236622],
     [-0.6785063, 0.7323368, 0.057552163, 7.133637],
     [-0.009389549, -0.086985044, 0.99616534, -0.20288628],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T06:02:31Z DEBUG registration_vulkan] Final RMSE: 0.4089983
[2026-03-04T06:02:31Z INFO  registration_vulkan] Total registration time: 584.01ms
[2026-03-04T06:02:31Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T06:02:31Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 295.375µs
[2026-03-04T06:02:31Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-40.pcd
Error: Failed to save results
```

# Radeon 780m
```bash
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] --- ICP Iteration 36 ---
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 51.086µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 275.228µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] Iter 35: RMSE = 0.409232, Valid points = 1765
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 134.604µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] --- ICP Iteration 37 ---
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 48.291µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 271.691µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] Iter 36: RMSE = 0.409136, Valid points = 1765
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 133.912µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] --- ICP Iteration 38 ---
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 74.621µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 274.235µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] Iter 37: RMSE = 0.409100, Valid points = 1765
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 131.507µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] --- ICP Iteration 39 ---
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 47.39µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 272.102µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] Iter 38: RMSE = 0.409043, Valid points = 1765
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 132.569µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] --- ICP Iteration 40 ---
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 46.026µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_search_neighbor] Compute neighbor search shader execution time: 271.741µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::registration] Iter 39: RMSE = 0.408998, Valid points = 1765
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_icp] Compute icp shader execution time: 138.03µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan] Final transformation matrix:
    [[0.73453456, 0.67536414, 0.06589619, 11.236621],
     [-0.6785063, 0.7323368, 0.057552166, 7.133637],
     [-0.009389553, -0.08698503, 0.9961652, -0.20288628],
     [0.0, 0.0, 0.0, 1.0]], shape=[4, 4], strides=[4, 1], layout=Cc (0x5), const ndim=2
[2026-03-04T06:16:09Z DEBUG registration_vulkan] Final RMSE: 0.40899828
[2026-03-04T06:16:09Z INFO  registration_vulkan] Total registration time: 442.70ms
[2026-03-04T06:16:09Z INFO  registration_vulkan] === Registration Results ===
[2026-03-04T06:16:09Z DEBUG registration_vulkan::gpu_transform] Compute transform shader execution time: 52.529µs
[2026-03-04T06:16:09Z DEBUG registration_vulkan::save_results] Saving aligned source and target point cloud to: data/output/debug/integrate-reverse-pattern/aligned-source-and-target_Rot_90_Z_iter-40.pcd
Error: Failed to save results

```