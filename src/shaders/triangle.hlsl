// Space[0,1,2] allocation follows https://wiki.libsdl.org/SDL3/SDL_CreateGPUShader
Texture2D Texture : register(t0, space2);
SamplerState Sampler : register(s0, space2);

cbuffer UniformBlock : register(b0, space1)
{
    float4x4 Matrix;
};

struct VsInput
{
    float3 Position : TEXCOORD0;
    float4 Color : TEXCOORD1;
    float2 TexCoord : TEXCOORD2;
    float4 mult_wash_fill_pad : TEXCOORD3;
};

struct VsOutput
{
    float2 TexCoord : TEXCOORD0;
    float4 Color : TEXCOORD1;
    float4 Position : SV_Position;
    float4 mult_wash_fill_pad : TEXCOORD3;
};

VsOutput vertex_main(VsInput input)
{
    VsOutput output;
    output.TexCoord = input.TexCoord;
    output.Color = input.Color;
    output.Position = mul(Matrix, float4(input.Position, 1.0));
    output.mult_wash_fill_pad = input.mult_wash_fill_pad;
    return output;
}

float4 fragment_main(VsOutput input) : SV_Target0
{
    float4 texture = Texture.Sample(Sampler, input.TexCoord);
    float4 color = input.Color;
    float mult = input.mult_wash_fill_pad.x;
    float wash = input.mult_wash_fill_pad.y;
    float fill = input.mult_wash_fill_pad.z;
    return mult * texture * color + wash * texture.a * color + fill * color;
}