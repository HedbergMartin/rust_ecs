
#![cfg_attr(feature = "unstable", feature(test))]

#[macro_use]
mod ecs;
pub use ecs::*;

#[cfg(all(feature = "unstable", test))]
#[path = "./benchmark/sparse_set_benchmark.rs"]
mod sparse_set_bench;

#[cfg(all(feature = "unstable", test))]
#[path = "./benchmark/cm_benchmark.rs"]
mod cm_bench;

#[cfg(all(feature = "unstable", test))]
#[path = "./benchmark/ecs_benchmark.rs"]
mod ecs_bench;