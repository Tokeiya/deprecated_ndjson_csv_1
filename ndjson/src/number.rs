#[derive(Debug)]
pub enum Number {
	Integer(i128),
	Float(f64),
}

impl From<i128> for Number {
	fn from(value: i128) -> Self {
		Number::Integer(value)
	}
}

impl From<f64> for Number {
	fn from(value: f64) -> Self {
		Number::Float(value)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn from_i128() {
		let fixture = Number::from(42);
		assert!(matches!(fixture,Number::Integer(i) if i==42));

		let fixture = Number::from(-42);
		assert!(matches!(fixture,Number::Integer(i) if i==-42));
	}

	#[test]
	fn from_f64() {
		let fixture = Number::from(std::f64::consts::PI);
		assert!(matches!(fixture, Number::Float(std::f64::consts::PI)));
	}
}
