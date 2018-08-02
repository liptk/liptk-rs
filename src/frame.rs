use ::Message;

enum_builder! {
	Type: u8;
	Name: FrameId;
	Variants {
		Message => 0x01
		// Tag => 0x??,
		// Digest => 0x??,
		// Signature => 0x??,
		// PublicKey => 0x??,
		// SecretKey => 0x??,
		// SharedKey => 0x??,
		// DerivedKey => 0x??,
		// SealedBox => 0x??,
		// SignedBox => 0x??,
		// TaggedBox => 0x??
	}
}

pub enum Frame {
	Message(Message),
}

impl Frame {
	pub fn id(&self) -> FrameId {
		match self {
			Frame::Message(_) => FrameId::Message,
		}
	}
}