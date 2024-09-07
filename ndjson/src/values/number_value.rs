use super::number::Number;
use super::text_presentation::TextPresentation;

pub struct NumberValue {
	number: Number,
	raw_string: String,
}

impl NumberValue {
	pub fn new(number: Number, raw_string: String) -> Self {
		Self { number, raw_string }
	}

	pub fn value(&self) -> &Number {
		&self.number
	}
}

impl TextPresentation for NumberValue {
	fn build_raw(&self, buffer: &mut String) {
		buffer.push_str(&self.raw_string)
	}

	fn build_trimmed(&self, buffer: &mut String) {
		buffer.push_str(self.raw_string.trim())
	}
}

#[cfg(test)]
mod test {
	use super::super::text_presentation::test_helper::*;
	use super::*;
	#[test]
	fn new() {
		let fixture = NumberValue::new(Number::from(42), "42".to_string());
		assert_eq!(&fixture.raw_string, "42");
		assert!(matches!(&fixture.number,Number::Integer(i) if i==&42i128));
	}

	#[test]
	fn value() {
		let fixture = NumberValue::new(
			Number::from(std::f64::consts::PI),
			std::f64::consts::PI.to_string(),
		);
		assert!(matches!(fixture.value(),&Number::Float(f) if f==std::f64::consts::PI));
	}
	#[test]
	fn text_presentation() {
		let fixture = NumberValue::new(
			Number::from(std::f64::consts::PI),
			format!("        {}\t\r\n", std::f64::consts::PI),
		);

		assert_trimmed(&fixture, std::f64::consts::PI.to_string().as_str());
		assert_raw(&fixture, &format!("        {}\t\r\n", std::f64::consts::PI));
	}
}
