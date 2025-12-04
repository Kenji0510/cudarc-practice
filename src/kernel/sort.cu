
extern "C" __global__ void sort_points(
    const float3* input_points,
    float3* output_sorted_points,
    const int* sorted_indices,
    int num_points
)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_points) return;

    int original_idx = sorted_indices[idx];

    output_sorted_points[idx] = input_points[original_idx];
}