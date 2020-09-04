
#![cfg_attr(feature = "unstable", feature(test))]

mod ecs;
pub use ecs::*;

#[cfg(test)]
#[path = "./tests/sparse_set_tests.rs"]
mod sparse_set_tests;

#[cfg(all(feature = "unstable", test))]
#[path = "./benchmark/sparse_set_benchmark.rs"]
mod sparse_set_bench;