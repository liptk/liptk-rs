use std::borrow::Cow;

use untrusted;

pub mod util;
pub mod error;

pub use self::error::Error;

pub type EncodeResult<'a> = self::error::Result<Cow<'a, [u8]>>;

pub trait Encodable<'a>: Sized {
	fn encode_to_bytes(&'a self) -> EncodeResult<'a>;
}

pub type DecodeResult<S> = self::error::Result<S>;

pub trait Decodable<'a>: Sized {
	fn decode_from_bytes(input: untrusted::Input<'a>) -> DecodeResult<Self>;
}