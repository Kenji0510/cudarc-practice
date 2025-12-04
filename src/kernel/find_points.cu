#include <cstdint>

__device__ uint64_t expand_bits(uint64_t v)
{
    v &= 0x1FFFFFull; 

    v = (v | (v << 32)) & 0x1F00000000FFFFull;
    v = (v | (v << 16)) & 0x1F0000FF0000FFull;
    v = (v | (v << 8))  & 0x100F00F00F00F00Full;
    v = (v | (v << 4))  & 0x10C30C30C30C30C3ull;
    v = (v | (v << 2))  & 0x1249249249249249ull;
    return v;
}

__device__ uint64_t morton3d(
    float3 point,
    float3 min_bound,
    float scale
)
{
    // Normalize point to [0, 1]
    float x_n = (point.x - min_bound.x) * scale;
    float y_n = (point.y - min_bound.y) * scale;
    float z_n = (point.z - min_bound.z) * scale;

    uint64_t ix = (uint64_t)fminf(fmaxf(x_n, 0.0f), 2097151.0f);
    uint64_t iy = (uint64_t)fminf(fmaxf(y_n, 0.0f), 2097151.0f);
    uint64_t iz = (uint64_t)fminf(fmaxf(z_n, 0.0f), 2097151.0f);

    return expand_bits(ix) | (expand_bits(iy) << 1) | (expand_bits(iz) << 2);
}

__device__ int binary_search(
    const uint64_t* data,
    int n,
    uint64_t val
)
{
    int l = 0;
    int r = n;
    while (l < r) {
        int mid = l + (r - l) / 2;
        if (data[mid] < val) {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    return l;
}

__device__ float distance_sq(float3 a, float3 b) {
    float dx = a.x - b.x;
    float dy = a.y - b.y;
    float dz = a.z - b.z;
    return dx*dx + dy*dy + dz*dz;
}

extern "C" __global__ void find_correspondence_points(
    const float3* query_points,  // source points
    const float3* sorted_ref_points, // sorted target points
    const uint64_t* sorted_ref_codes, // sorted target Morton codes
    int* output_indices,
    float* output_dists,
    int num_query_points,
    int num_ref, // number of target points
    float3 min_bound,
    float scale,
    int search_windowsize // search window size
)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_query_points) return;

    float3 q_point = query_points[idx];

    uint64_t q_code = morton3d(q_point, min_bound, scale);

    int center_idx = binary_search(sorted_ref_codes, num_ref, q_code);

    int start_idx = max(0, center_idx - search_windowsize);
    int end_idx = min(num_ref, center_idx + search_windowsize);

    float min_dist_sq = 1e20f;
    int best_idx = -1;

    for (int i = 0; i < end_idx; i++) {
        float3 ref_point = sorted_ref_points[i];
        float d2 = distance_sq(q_point, ref_point);

        if (d2 < min_dist_sq) {
            min_dist_sq = d2;
            best_idx = i;
        }
    }

    output_indices[idx] = best_idx;
    output_dists[idx] = min_dist_sq;
}