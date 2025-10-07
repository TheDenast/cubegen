# `cubegen`

> Rust voxel graphics / terrain playground

## Project reference

Note: suff below is for my own future reference and was shamelessly generated with LLM. Beware, slop!

### Core Choices

- **Language**: Rust (learning opportunity + systems programming skills)
- **Graphics**: wgpu (modern abstraction, avoids Vulkan ceremony)
- **Platform**: Linux

### Why Not Vulkan

- 3-4 weeks pure setup before rendering anything
- 90% API memorization, not engineering challenges
- Doesn't align with "solve problems, not memorize APIs" goal

### Why wgpu

- Reasonable abstraction level - still low-level concepts
- Good documentation, mature ecosystem
- Focus time on actual engineering (meshing, streaming, optimization)
- Graphics API fades into background after initial setup

### Learning Approach

- Skip rustlings, learn Rust while building
- Context-driven learning > isolated exercises
- Use compiler errors as teacher
- Keep Rust book as reference, not sequential reading

### Project Structure

1. **Phase 1**: Build graphics foundation/wrappers (before any game logic)
2. **Phase 2**: Game systems (long-term goal)

### Skills Transfer

- Rust ownership → distributed systems thinking
- Memory management → infrastructure work
- Systems design patterns → general engineering
- Not graphics-specific learning, systems programming focus
