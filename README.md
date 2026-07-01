# Rust + wgpu Summer Project

A self-directed summer deep-dive into systems programming and GPU rendering, two areas well outside my usual frontend work.

## Why this exists

Most of my experience lives in the browser: JavaScript, TypeScript, React, CSS. That stack abstracts away nearly everything about how a computer actually runs code: the memory model, the hardware, the pipeline between CPU and GPU.

This project is an intentional step off that abstraction ladder. The goal isn't to ship a product. It's to understand what's underneath.

## What I'm learning

### Rust
Rust forces you to think about things JavaScript never asks you to consider:
- **Ownership & borrowing**: who holds data, who can read it, who can mutate it, and when it gets freed. There's no garbage collector covering your tracks.
- **Types at compile time**: errors that would silently blow up at runtime in JS are caught before the program ever runs.
- **Explicit error handling**: no surprise exceptions; every failure path is something you have to decide what to do with.

### wgpu
wgpu is a cross-platform GPU API written in Rust, built on top of the same ideas as Vulkan, Metal, and DirectX 12. Working with it means:
- Writing **shaders**, small programs that run on the GPU, not the CPU.
- Managing the **render pipeline** manually: buffers, bind groups, swap chains, passes.
- Understanding the **CPU/GPU boundary**: what data lives where, how it gets transferred, when each side does its work.

This is as far from `<canvas>` and WebGL convenience wrappers as it gets.

---

## Completed

- [x] Rust toolchain installed, project structure set up
- [x] Guessing game: variables, loops, match, error handling, external crates (`rand`)
- [x] Rustlings: intro, variables, functions sections

---

## Roadmap

### Month 1: Rust Foundations (July)

Goal: get comfortable enough with Rust that the language stops being the obstacle.

**Core language (via Rustlings + The Rust Book)**
- [ ] Control flow: `if`, `loop`, `while`, `for`, ranges
- [ ] Ownership and borrowing: the borrow checker, references, slices
- [ ] Structs: defining, instantiating, methods, associated functions
- [ ] Enums and pattern matching: `Option`, `Result`, `match`, `if let`
- [ ] Collections: `Vec`, `String`, `HashMap`
- [ ] Modules and crates: `mod`, `use`, visibility, organising a multi-file project
- [ ] Error handling: `?` operator, custom error types, `thiserror`
- [ ] Traits and generics: defining traits, implementing them, trait bounds, `impl Trait`
- [ ] Lifetimes: what they are, when the compiler asks for them, common patterns
- [ ] Closures and iterators: `map`, `filter`, `fold`, chaining, lazy evaluation
- [ ] Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, when to reach for each

**Small projects to cement the above**
- [ ] A CLI todo list (structs, enums, file I/O, error handling)
- [ ] A simple text parser (iterators, string manipulation, pattern matching)

**Resources**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings exercises](https://github.com/rust-lang/rustlings)

---

### Month 2: Into wgpu (August)

Goal: get something rendering on screen and understand every line of why.

Follow the [Learn wgpu](https://sotrh.github.io/learn-wgpu/) tutorial end to end.

**Getting a window and a surface**
- [ ] Set up a `winit` window and `wgpu` instance
- [ ] Create a surface, adapter, device, and queue
- [ ] Configure the swap chain and render a solid colour

**The render pipeline**
- [ ] Write your first WGSL vertex and fragment shaders
- [ ] Create a pipeline with a shader module, vertex layout, and colour target
- [ ] Render a triangle using a vertex buffer
- [ ] Add an index buffer to reuse vertices

**Textures and transforms**
- [ ] Load an image as a texture, create a sampler, set up a bind group
- [ ] Pass data to shaders with uniform buffers
- [ ] Implement a basic 3D camera (view and projection matrices)
- [ ] Move objects using transformation matrices

**Depth and instancing**
- [ ] Add a depth buffer to handle occlusion correctly
- [ ] Use instancing to render many objects in a single draw call

**Model loading**
- [ ] Load an OBJ model with materials
- [ ] Render it with lighting

**Resources**
- [Learn wgpu](https://sotrh.github.io/learn-wgpu/) (primary)
- [wgpu-step-by-step by jack1232](https://github.com/jack1232/wgpu-step-by-step)
- [wgpu docs](https://docs.rs/wgpu/)

---

### Month 3: Going Further (September)

Goal: apply everything to a self-directed project with no tutorial to follow.

**Advanced rendering (pick what fits your project)**
- [ ] Lighting: diffuse, specular, point lights, Phong shading
- [ ] Normal mapping for surface detail without added geometry
- [ ] High dynamic range (HDR) rendering and tone mapping
- [ ] Mipmapping and stencil buffers

**Compute shaders**
- [ ] Understand the compute pipeline vs the render pipeline
- [ ] Write a WGSL compute shader, dispatch it, read the result back
- [ ] Use storage buffers to pass large data sets to the GPU

**Self-directed project (choose one)**
- [ ] 2D sprite renderer
- [ ] Simple 3D scene with lighting and a free camera
- [ ] Particle system driven by a compute shader
- [ ] Procedural terrain with a heightmap

The project should use no tutorial. The goal is to manage your own buffers, pipelines, and bind groups from scratch.

---

## Structure

```
guessing_game/   first Rust project; CLI number guessing game
rustlings/       Rust language exercises
```

wgpu work coming as Rust fundamentals solidify.

## The point

Frontend skills are real and valuable. But spending a summer here builds intuition that makes everything else sharper: why rendering is expensive, why memory allocation matters, why the languages and tools that power the web are designed the way they are.
