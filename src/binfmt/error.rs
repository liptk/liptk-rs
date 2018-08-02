use untrusted::EndOfInput;

error_chain! {
	foreign_links {
		Io(::std::io::Error);
	}
	errors {
		Unexpected {
			description("unexpected")
			display("unexpected")
		}
		EndOfInput {
			description("end of input")
			display("end of input")
		}
	}
}

impl From<EndOfInput> for Error {
	fn from(_: EndOfInput) -> Self {
		Self::from_kind(ErrorKind::EndOfInput)
	}
}
