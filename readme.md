
# Game Engine (WIP) 

## A small 2D/3D Rust Game Framework, using few dependencies and simple code to maintain easy building and portability.

- Platform Support: Currently tested on macOS/Metal (cross-compilation via shadercross for other backends).
- Dependencies: Minimal (SDL3, nalgebra, stb_image).

### Features:

- Hot-reloadable game DLLs for rapid iteration
- Sprite batching to reduce draw calls
- Multiple shader/material support
- Custom immediate-mode GUI
- Custom bitmap global allocator (WIP)
- Roadmap: Entity Component System (ECS) integration

![demo2](demo2.jpg)
![demo](demo.jpg)