# Rust_ECS

## Description
Rust_ECS is an open-source Entity-Component-System(ECS) written in Rust. Its main goal is to be easy to use while still having high performance. 

Rust_ECS is an sparse set entity compontent system inspired by ENTT.

## WARNING
Rust_ecs is in a very early stage. When it's in a state ready to be used, I will remove this warning :)

## Currently implemented:
(Exluding spawning, destroying and iterating of entities/components)
* Entitys
    * Resource reusing
* Components
    * Inline memory storage for minimum cache misses
    * No overhead tight grouping

## How to use

### As library:
Coming soon..

### Run tests:
cargo test

### Run benchmark:
rustup run nightly cargo bench --features unstable

## About
Rust_ECS is currently developed solely by me as a hobby project in order to get more familiar with Rust. My goal is to be able to use it in my own projects down the line.