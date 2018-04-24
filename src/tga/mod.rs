//! Decoding of TGA Images
//!
//! # Related Links
//! <http://googlesites.inequation.org/tgautilities>

/// A decoder for TGA images
///
/// Currently this decoder does not support 8, 15 and 16 bit color images.
pub use self::decoder::TGADecoder;

mod decoder;
