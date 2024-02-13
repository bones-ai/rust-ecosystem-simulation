# Procedural World Generation
This is a simple boid ecosystem simulation inspired by Daniel Shiffman's (TheCodingTrain) Ecosystem simulation - https://www.youtube.com/watch?v=flxOkx0yLrY

Built in [Rust](https://www.rust-lang.org/) using the [Bevy](https://bevyengine.org/) game engine.

![screenshot](/screenshot.png)

## Timelapse video

### Part 1
[![youtube](https://img.youtube.com/vi/lCUovKa68jQ/0.jpg)](https://youtu.be/lCUovKa68jQ)

### Part 2 with predators
[![youtube](https://img.youtube.com/vi/sKYUIlDdC18/0.jpg)](https://youtu.be/sKYUIlDdC18)

## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/rust-ecosystem-simulation.git
cd rust-ecosystem-simulation
```
- Run the simulation
```bash
cargo run
```

## Controls
- `Backspace` - Show graphs
- `Tilde` - Show graph settings
- `Tab` - Show debug gizmos
- `1` - Camera follow boid
- `2` - Camera follow predator boid
- `3` - Camera snap to center

## Configurations
- The project config file is located at `src/configs.rs`

## Local Development

### Step #1: Install Dependencies

Execute `cargo build` in your terminal

### Step #2: Run the Binary

Execute `cargo run` to start the project