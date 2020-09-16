# Rust_ECS
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/HedbergMartin/rust_ecs/blob/master/LICENSE)

## Description
Rust_ECS is an open-source Entity-Component-System(ECS) written in Rust. Its main goal is to be easy to use while still having high performance. 

Rust_ECS is an sparse set entity compontent system inspired by ENTT.

## WARNING
Rust_ECS is in a very early stage. When it's in a state ready to be used, I will remove this warning :)

## Currently implemented:
(Exluding spawning, destroying and iterating of entities/components)
* Entitys
    * Resource reusing
* Components
    * Inline memory storage for minimum cache misses
    * No overhead tight grouping

## How to use

### As library:

Add this line under [dependencies] in your Cargo.toml to include Rust_ECS in your project.

```
rust_ecs = { git = "https://github.com/HedbergMartin/rust_ecs.git" }
```

Then add this to to your crate root file:

```rust
#[macro_use]
extern crate rust_ecs;
```


### Run tests:
```
cargo test
```

### Run benchmark:
Warning, benchmarks are not recommended to run in current state as there design is flawed.
```
rustup run nightly cargo bench --features unstable
```

## About
Rust_ECS is currently developed solely by me as a hobby project in order to get more familiar with Rust. My goal is to be able to use it in my own projects down the line.
