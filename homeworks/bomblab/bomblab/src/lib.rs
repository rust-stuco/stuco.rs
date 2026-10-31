#![doc = include_str!("../README.md")]

mod bomb;
pub use bomb::Bomb;

mod debug_vec;
pub use debug_vec::DebugVec;

pub mod defuser;

#[cfg(test)]
mod tests;
