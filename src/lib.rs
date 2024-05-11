#![doc = include_str!("../README.md")]
mod api;
mod curies;
mod trie;

pub use api::{CurieParts, CuriePrefix, CurieUtil};
pub use trie::TrieCurieUtil;
