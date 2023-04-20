//! # liteyear
//!
//! ## Example
//!
//! ```
//! #[liteyear::bench(n = 1..=8)]
//! fn fac(n: u32) -> u32 {
//!     match n {
//!         0 | 1 => 1,
//!         n => n * fac(n - 1),
//!     }
//! }
//! ```

pub use crate::benchmark::*;
pub use liteyear_macro::*;

pub mod benchmark;
