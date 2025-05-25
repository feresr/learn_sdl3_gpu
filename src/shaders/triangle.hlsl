// Vertex shader
struct vertex_info
{
    float3 position : POSITION;
    float3 color : COLOR;
};

struct vertex_to_pixel
{
    float4 position : SV_POSITION;
    float3 color : COLOR;
};

vertex_to_pixel vertex_main(in vertex_info IN)
{
    vertex_to_pixel OUT;

    OUT.position = float4(IN.position, 1.0);
    OUT.color = IN.color;

    return OUT;
}

// Fragment shader
struct input_from_vertex
{
    float3 color : COLOR;
};

float4 fragment_main(in input_from_vertex IN) : SV_TARGET0
{
    return float4(IN.color, 1.0);
}