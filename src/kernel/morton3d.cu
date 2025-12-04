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

extern "C" __global__ void compute_morton_codes(
    const float3* points,
    uint64_t* morton_code,
    int* indices,
    int num_points,
    float3 min_bound,
    float scale
)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_points) return;

    float3 p = points[idx];

    morton_code[idx] = morton3d(p, min_bound, scale);

    indices[idx] = idx;
}
