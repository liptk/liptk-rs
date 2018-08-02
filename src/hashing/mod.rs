//! # Hashing
//! 
//! A hashing algorithm and it's parameters compute a digest of fixed size 
//! for an input of arbitrary size.
//! 
//! All hashing algorithms available in this module are cryptographically secure.
//! 
//! For more information on the hashing algorithms available in this module, and
//! how to use them securely see the [LIPTK book](TODO).

enum_builder! {
	Type: u8;
	Name: HashingAlgId;
	Variants {
		/// Secure Hash Algorithm (256 bit digests)
		SHA256 => 0x01,
		/// Secure Hash Algorithm (512 bit digests)
		SHA512 => 0x02
	}
}

#[derive(Debug, Clone)]
pub struct Digest {
	alg_id: HashingAlgId,
	bytes: Vec<u8>
}