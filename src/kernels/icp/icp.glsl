#version 450

const uint BLOCK_SIZE = 64;
layout(local_size_x = BLOCK_SIZE, local_size_y = 1, local_size_z = 1) in;

layout(push_constant) uniform IcpParams {
    int num_source;
    int num_target;
    float max_dist_sq;
} params;

layout(set = 0, binding = 0) readonly buffer SourcePts { float source_pts[]; };
layout(set = 0, binding = 1) readonly buffer TargetPts { float target_pts[]; };
layout(set = 0, binding = 2) readonly buffer TargetNormals { float target_normals[]; };
layout(set = 0, binding = 3) readonly buffer Indices { int indices[]; };
layout(set = 0, binding = 4) readonly buffer Distances { float dists_sq[]; };

layout(set = 0, binding = 5) buffer OutH { uint d_H[]; };   // H is 6x6 = 36 floats, stored in row-major order
layout(set = 0, binding = 6) buffer OutB { uint d_b[]; };   // b is 6x1 = 6 floats

void atomicAddH(uint index, float val) {
    uint assumed;
    uint old_val = d_H[index];
    do {
        assumed = old_val;
        float new_float = uintBitsToFloat(assumed) + val;
        uint new_val = floatBitsToUint(new_float);
        old_val = atomicCompSwap(d_H[index], assumed, new_val);
    } while (assumed != old_val);
}

void atomicAddB(uint index, float val) {
    uint assumed;
    uint old_val = d_b[index];
    do {
        assumed = old_val;
        float new_float = uintBitsToFloat(assumed) + val;
        uint new_val = floatBitsToUint(new_float);
        old_val = atomicCompSwap(d_b[index], assumed, new_val);
    } while (assumed != old_val);
}

shared float s_H[BLOCK_SIZE * 36];
shared float s_b[BLOCK_SIZE * 6];

void main() {
    uint gid = gl_GlobalInvocationID.x;
    uint lid = gl_LocalInvocationID.x;

    float local_H[36];
    float local_b[6];

    for (int i = 0; i < 36; i++) local_H[i] = 0.0;
    for (int i = 0; i < 6; i++) local_b[i] = 0.0;

    if (gid < uint(params.num_source)) {
        int target_idx = indices[gid];
        float dist_sq = dists_sq[gid];

        if (target_idx >= 0 && uint(target_idx) < uint(params.num_target) && dist_sq <= params.max_dist_sq) {
            uint src_offset = gid * 3;
            float ps_x = source_pts[src_offset + 0];
            float ps_y = source_pts[src_offset + 1];
            float ps_z = source_pts[src_offset + 2];

            uint tgt_offset = uint(target_idx) * 3;
            float pt_x = target_pts[tgt_offset + 0];
            float pt_y = target_pts[tgt_offset + 1];
            float pt_z = target_pts[tgt_offset + 2];

            float nt_x = target_normals[tgt_offset + 0];
            float nt_y = target_normals[tgt_offset + 1];
            float nt_z = target_normals[tgt_offset + 2];

            float cross_x = ps_y * nt_z - ps_z * nt_y;
            float cross_y = ps_z * nt_x - ps_x * nt_z;
            float cross_z = ps_x * nt_y - ps_y * nt_x;

            float J[6];
            J[0] = cross_x; J[1] = cross_y; J[2] = cross_z;
            J[3] = nt_x;    J[4] = nt_y;    J[5] = nt_z;

            float diff_x = pt_x - ps_x;
            float diff_y = pt_y - ps_y;
            float diff_z = pt_z - ps_z;
            float res = diff_x * nt_x + diff_y * nt_y + diff_z * nt_z;

            for (int i = 0; i < 6; i++) {
                local_b[i] += J[i] * res;
            }

            for (int r = 0; r < 6; r++) {
                float Jr = J[r];
                for (int c = 0; c < 6; c++) {
                    local_H[r * 6 + c] += Jr * J[c];
                }
            }
        }
    }

    uint baseH = lid * 36;
    uint baseB = lid * 6;

    for (int i = 0; i < 36; i++) s_H[baseH + i] = local_H[i];
    for (int i = 0; i < 6; i++)  s_b[baseB + i] = local_b[i];

    barrier();

    for (uint offset = BLOCK_SIZE >> 1; offset > 0; offset >>= 1) {
        if (lid < offset) {
            uint otherH = (lid + offset) * 36;
            uint otherB = (lid + offset) * 6;

            for (int i = 0; i < 36; i++) {
                s_H[baseH + i] += s_H[otherH + i];
            }
            for (int i = 0; i < 6; i++) {
                s_b[baseB + i] += s_b[otherB + i];
            }
        }
        barrier();
    }

    if (lid == 0) {
        for (int i = 0; i < 36; i++) {
            if (s_H[baseH + i] != 0.0) {
                atomicAddH(uint(i), s_H[baseH + i]);
            }
        }
        for (int i = 0; i < 6; i++) {
            if (s_b[baseB + i] != 0.0) {
                atomicAddB(uint(i), s_b[baseB + i]);
            }
        }
    }
}