pub mod slice {
	pub fn u8_be_from_u16(value: u16) -> [u8; 2] {
		[((value >> 8) & 0xff) as u8, (value & 0xff) as u8]
	}
}