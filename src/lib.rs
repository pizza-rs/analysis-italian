#![cfg_attr(not(feature = "std"), no_std)]
//! Italian language analysis for Pizza search engine.
//!
//! Provides a full-featured Italian analyzer with elision removal,
//! light stemming, and stop words.
extern crate alloc;
mod elision;
mod stem;
mod stop;

pub mod register;

pub use elision::ItalianElisionFilter;
pub use register::register_all;
pub use stem::ItalianLightStemFilter;
pub use stop::ItalianStopFilter;
