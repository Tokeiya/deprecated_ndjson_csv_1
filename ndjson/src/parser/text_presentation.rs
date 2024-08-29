use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TextPresentation {
	Character(char),
	Text(String),
	Empty,
}

impl From<char> for TextPresentation {
	fn from(value: char) -> Self {
		TextPresentation::Character(value)
	}
}

impl From<&str> for TextPresentation {
	fn from(value: &str) -> Self {
		if value.is_empty() {
			TextPresentation::Empty
		} else {
			TextPresentation::Text(value.to_string())
		}
	}
}

impl From<String> for TextPresentation {
	fn from(value: String) -> Self {
		if value.is_empty() {
			TextPresentation::Empty
		} else {
			TextPresentation::Text(value)
		}
	}
}

impl Display for TextPresentation {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			TextPresentation::Character(c) => write!(f, "{c}"),
			TextPresentation::Text(x) => write!(f, "{x}"),
			TextPresentation::Empty => Ok(()),
		}
	}
}

impl TextPresentation {
	pub fn write_to_buffer(&self, buffer: &mut String) {
		match self {
			TextPresentation::Character(c) => buffer.push(*c),
			TextPresentation::Text(s) => buffer.push_str(s),
			TextPresentation::Empty => {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn from_string() {
		let s = "hello".to_string();
		let fixture = TextPresentation::from(s);

		assert!(matches!(
			fixture,TextPresentation::Text(s) if s=="hello"
		));

		let fixture = TextPresentation::Text("".to_string());
		println!("{fixture:?}");

		assert!(matches!(
			TextPresentation::from("".to_string()),
			TextPresentation::Empty
		));

		assert!(matches!(TextPresentation::from(" ".to_string()),
			TextPresentation::Text(t) if t==" "));
	}
	#[test]
	fn from_char() {
		let fixture = TextPresentation::from('a');
		assert!(matches!(fixture,TextPresentation::Character(c) if c=='a'))
	}

	#[test]
	fn from_ref_str() {
		assert!(matches!(TextPresentation::from("hello"),TextPresentation::Text(s) if s=="hello"));

		assert!(matches!(
			TextPresentation::from(""),
			TextPresentation::Empty
		));

		assert!(matches!(TextPresentation::from(" "),
			TextPresentation::Text(t) if t==" "));
	}
	#[test]
	fn debug() {
		let fixture = TextPresentation::from("hello");
		let act = format!("{fixture:?}");
		assert_eq!(&act, r#"Text("hello")"#);

		let fixture = TextPresentation::from('b');
		let act = format!("{fixture:?}");
		assert_eq!(&act, r#"Character('b')"#);

		assert_eq!(format!("{:?}", TextPresentation::Empty), "Empty")
	}
	#[test]
	fn display() {
		let fixture = format!("{}", TextPresentation::from('a'));
		assert_eq!(fixture, "a");

		let fixture = format!("{}", TextPresentation::from("hello"));
		assert_eq!(fixture, "hello");

		assert_eq!(format!("{}", TextPresentation::Empty), "");
	}
	#[test]
	fn append() {
		let mut buff = String::from("hello ");

		TextPresentation::Empty.write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello ");

		let fixture = TextPresentation::from("world");
		fixture.write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world");

		TextPresentation::from(' ').write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world ");

		TextPresentation::from('r').write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world r");

		TextPresentation::from('u').write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world ru");

		TextPresentation::from("s").write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world rus");

		TextPresentation::from("t").write_to_buffer(&mut buff);
		assert_eq!(&buff, "hello world rust");
	}
}
