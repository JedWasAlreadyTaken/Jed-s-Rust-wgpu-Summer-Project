# Rust + wgpu Summer Project

I'm a frontend developer spending the summer learning Rust and GPU rendering from scratch. This repo tracks my progress.

## Background

I work mostly in JavaScript and TypeScript. This is me going deliberately off-script and learning how things work closer to the metal: memory management, the type system, and eventually writing code that runs directly on the GPU.

---

## Progress

- [x] Rust toolchain installed, project structure set up
- [x] Guessing game: variables, loops, match, error handling, external crates (`rand`)
- [x] Rustlings: intro, variables, functions sections

---

## Roadmap

### Month 1: Rust Foundations (July)

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
- [ ] Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`

**Small projects**
- [ ] CLI todo list (structs, enums, file I/O, error handling)
- [ ] Simple text parser (iterators, string manipulation, pattern matching)

**Resources**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

### Month 2: Into wgpu (August)

Working through [Learn wgpu](https://sotrh.github.io/learn-wgpu/) end to end.

**Window and surface**
- [ ] Set up a `winit` window and `wgpu` instance
- [ ] Create a surface, adapter, device, and queue
- [ ] Configure the swap chain and render a solid colour

**Render pipeline**
- [ ] Write WGSL vertex and fragment shaders
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
- [wgpu-step-by-step](https://github.com/jack1232/wgpu-step-by-step)
- [wgpu docs](https://docs.rs/wgpu/)

---

### Month 3: Build something (September)

No tutorial. Pick a project and build it from scratch.

**Advanced topics to pull from as needed**
- [ ] Lighting: diffuse, specular, point lights, Phong shading
- [ ] Normal mapping
- [ ] HDR rendering and tone mapping
- [ ] Mipmapping and stencil buffers
- [ ] Compute shaders: write a WGSL compute shader, dispatch it, read the result back
- [ ] Storage buffers for large GPU data sets

**Project options**
- [ ] 2D sprite renderer
- [ ] 3D scene with lighting and a free camera
- [ ] Particle system driven by a compute shader
- [ ] Procedural terrain with a heightmap

---

## Repo structure

```
guessing_game/   CLI number guessing game (first Rust project)
rustlings/       Rust language exercises
```
