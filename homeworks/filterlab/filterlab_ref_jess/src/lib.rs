#![doc = include_str!("../README.md")]

mod bitvector_vanilla;
mod bloomfilter;

#[cfg(not(target_arch = "x86_64"))]
mod doublehasher;

#[cfg(target_arch = "x86_64")]
mod doublehasher_x86_avx;

pub use bloomfilter::BloomFilter;
