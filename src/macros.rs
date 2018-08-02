macro_rules! enum_builder {
	(
		Type: $enum_code_type: ty;
		Name: $enum_name: ident;
		Variants {
			$(
				$(#[$attr:meta])*
				$enum_var: ident => $enum_val: expr
			),*
		}
	) => {
		#[allow(non_camel_case_types)]
		#[derive(Debug, PartialEq, Eq, Clone, Copy)]
		pub enum $enum_name {
			$( $enum_var ),*
			,Unknown($enum_code_type)
		}
		impl From<$enum_code_type> for $enum_name {
			fn from(code: $enum_code_type) -> Self {
				match code {
					$( $enum_val => $enum_name::$enum_var ),*
					,x => $enum_name::Unknown(x)
				}
			}
		}
		impl From<$enum_name> for $enum_code_type {
			fn from(s: $enum_name) -> $enum_code_type {
				let x = s.clone();
				match x {
					$( $enum_name::$enum_var => $enum_val ),*
					,$enum_name::Unknown(x) => x
				}
			}
		}
	}
}
