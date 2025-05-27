// Vertex shader
struct Input
{
    float3 Position : TEXCOORD0;
    float3 TexCoord : TEXCOORD1;
};

struct Output
{
    float3 TexCoord : TEXCOORD0;
    float4 Position : SV_Position;
};

Output vertex_main(Input input)
{
    Output output;
    output.TexCoord = input.TexCoord;
    output.Position = float4(input.Position, 1.0f);
    return output;
}

// Fragment shader
Texture2D<float4> Texture : register(t0, space0);
SamplerState Sampler : register(s0, space0);

float4 fragment_main(float3 TexCoord : TEXCOORD0) : SV_Target0
{
    return Texture.Sample(Sampler, TexCoord.xy);
}
