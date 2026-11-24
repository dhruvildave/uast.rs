//! Rust implementation of [Unicode Aware Saṃskṛta Transliteration](https://arxiv.org/html/2203.14277).
//!
//! The following Unicode code-blocks are used:
//! - देवनागरी: <https://www.unicode.org/charts/PDF/U0900.pdf>
//! - ગુજરાતી: <https://www.unicode.org/charts/PDF/U0A80.pdf>
//!
//! <div class="warning">
//! No Unicode normalization is performed. It is assumed that the text is already normalized.
//! </div>
//!
//! # Output
//!
//! Each function returns only characters from its conversion tables, with words separated by a
//! single ASCII space.
//!
//! | Function | Output characters |
//! | --- | --- |
//! | `uast_to_devanāgarī` | देवनागरी letters, signs, digits and punctuation, and Vedic accents |
//! | `devanāgarī_to_iast` | IAST letters, ASCII digits, `'`, `.`, `-`, `ॐ` and `॰` |
//! | `slp_to_iast` | IAST letters, ASCII digits, `'` and `.` |
//! | `devanāgarī_to_gujarātī` | ગુજરાતી letters, signs and digits, `।` and `॥` |
//!
//! Characters without a mapping are dropped silently, such as zero-width joiners and the combining
//! marks of decomposed (NFD) text. `uast_to_devanāgarī` and `devanāgarī_to_iast` lowercase the
//! input before converting it.
//!
//! A filter on the input does not limit what the output contains, so encode the output for the
//! place you use it, for example by escaping it before inserting it into HTML. IAST output can
//! contain an ASCII apostrophe, and a word in it can be just `.` or `..`.
//!
//! The functions do not limit input length, and memory use grows with the input. If you convert
//! untrusted text in a shared process, cap the input length first.

mod gu;
mod iast;
mod slp;
#[cfg(test)]
mod tests;
mod uast;
mod utils;

pub use crate::gu::devanāgarī_to_gujarātī;
pub use crate::iast::devanāgarī_to_iast;
pub use crate::slp::slp_to_iast;
pub use crate::uast::uast_to_devanāgarī;
