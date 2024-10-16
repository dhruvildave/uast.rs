//! Rust implementation of [Unicode Aware Saṃskṛta Transliteration](https://arxiv.org/html/2203.14277).
//!
//! The following Unicode code-blocks are used:
//! - देवनागरी: <https://www.unicode.org/charts/PDF/U0900.pdf>
//! - ગુજરાતી: <https://www.unicode.org/charts/PDF/U0A80.pdf>
//!
//! __Warning__: No Unicode normalization is performed. It is assumed that the text is already normalized.

mod gu;
mod iast;
#[cfg(test)]
mod tests;
mod uast;
mod utils;

pub use crate::gu::devanāgarī_to_gujarātī;
pub use crate::iast::devanāgarī_to_iast;
pub use crate::uast::uast_to_devanāgarī;
