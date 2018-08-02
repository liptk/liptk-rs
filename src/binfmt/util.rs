use untrusted;
use byteorder::{ByteOrder, BigEndian};

use polyfill::slice::u8_be_from_u16;

use super::error;

pub fn read_all<'a, R, F>(input: untrusted::Input<'a>, read: F) -> error::Result<R>
where
	F: FnOnce(&mut untrusted::Reader<'a>) -> Result<R, error::Error>,
{
	input.read_all(error::ErrorKind::Unexpected.into(), read)
}

pub fn read_u16_from_be_u8<'a>(reader: &mut untrusted::Reader<'a>) -> error::Result<u16> {
	let input = reader.skip_and_get_input(2)?;
	Ok(BigEndian::read_u16(input.as_slice_less_safe()))
}

pub fn be_u8_from_usize_to_u16(value: usize) -> error::Result<[u8; 2]> {
	if value > u16::max_value() as usize {
		Err(error::ErrorKind::Unexpected.into())
	} else {
		Ok(u8_be_from_u16(value as u16))
	}
}