__constant__ float coefficients[4];

extern "C" __global__ void polynomial_kernel(
    float *out,
    const float *inp,
    int numel
) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i < numel) {
        float x = inp[i];
        out[i] = coefficients[0] + coefficients[1] * x
            + coefficients[2] * x * x
            + coefficients[3] * x * x * x;
    }
}