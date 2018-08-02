//! # KDF (Key Derivation Function)
//! 
//! A KDF algorithm and it's parameters, given a master key, will deterministically
//! derive a child/derived key of an arbitrary size.
//! 
//! - KDFs are commonly used with shared-key as the master to generate a key for a given 
//!   purpose, as opposed to using the shared-key directly.
//! - KDF algorithms also have properties that can make them excellent for password-hashing.
//! 
//! All KDF algorithms available in this module are cryptographically secure.
//! 
//! For more information on the KDF algorithms available in this module, and
//! how to use them securely see the [LIPTK book](TODO).

enum_builder! {
	Type: u8;
	Name: KdfAlgId;
	Variants {
		/// Password-Based Key Derivation Function 2
		/// (with SHA256 as the hashing function)
		PBKDF2_SHA256 => 0x01,
		/// Password-Based Key Derivation Function 2
		/// (with SHA512 as the hashing function)
		PBKDF2_SHA512 => 0x02,
		/// HMAC-based Extract-and-Expand Key Derivation Function
		/// (with SHA256 as the hashing function)
		HKDF_SHA256 => 0x03,
		/// HMAC-based Extract-and-Expand Key Derivation Function
		/// (with SHA512 as the hashing function)
		HKDF_SHA512 => 0x04
	}
}

/// Public parameters used when deriving a key.
/// 
/// Public in this context typically relates to parameters like a salt.
#[derive(Debug, Clone)]
pub enum KdfParams {
	// TODO
}

/// The key produced from a KDF algorithm.
#[derive(Debug, Clone)]
pub struct DerivedKey {
	alg_id: KdfAlgId,
	params: Option<KdfParams>,
	bytes: Vec<u8>
}
