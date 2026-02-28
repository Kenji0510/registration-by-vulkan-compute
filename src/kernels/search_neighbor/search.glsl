#version 450

layout(local_size_x = 256, local_size_y = 1, local_size_z = 1) in;

layout(push_constant) uniform SearchParams {
    uint num_source;
    uint num_target;
} params;

layout(set = 0, binding = 0) readonly buffer SourcePts {
    float source_pts[];
};

layout(set = 0, binding = 1) readonly buffer TargetPts {
    float target_pts[];
};

layout(set = 0, binding = 2) writeonly buffer OutIndices {
    int out_indices[];
};

layout(set = 0, binding = 3) writeonly buffer OutDistsSq {
    float out_dists_sq[];
};

void main() {
    uint idx = gl_GlobalInvocationID.x;

    if (idx >= params.num_source) return;

    uint src_offset = idx * 3;
    float px = source_pts[src_offset + 0];
    float py = source_pts[src_offset + 1];
    float pz = source_pts[src_offset + 2];

    float best_dist_sq = 1.0e30;
    int best_target_idx = -1;

    for (uint j = 0; j < params.num_target; j++) {
        uint tgt_offset = j * 3;
        float tx = target_pts[tgt_offset + 0];
        float ty = target_pts[tgt_offset + 1];
        float tz = target_pts[tgt_offset + 2];

        float dx = px - tx;
        float dy = py - ty;
        float dz = pz - tz;

        float dist_sq = dx * dx + dy * dy + dz * dz;

        if (dist_sq < best_dist_sq) {
            best_dist_sq = dist_sq;
            best_target_idx = int(j);
        }
    }

    out_dists_sq[idx] = best_dist_sq;
    out_indices[idx] = best_target_idx;
}