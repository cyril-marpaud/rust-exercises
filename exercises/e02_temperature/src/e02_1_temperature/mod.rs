pub struct Temp {}

pub enum TempUnit {}

impl Temp {
	pub fn new() {}
	pub fn convert() {}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn c_to_f() {
		let mut t = Temp::new(0.0, TempUnit::Celsius);
		assert_eq!(t.convert(TempUnit::Fahrenheit).temp, 32.0);
		assert!(matches!(t.unit, TempUnit::Fahrenheit));

		let mut t = Temp::new(50.5, TempUnit::Celsius);
		assert_eq!(t.convert(TempUnit::Fahrenheit).temp, 122.9);
		assert!(matches!(t.unit, TempUnit::Fahrenheit));
	}

	#[test]
	fn c_to_k() {
		let mut t = Temp::new(0.0, TempUnit::Celsius);
		assert_eq!(t.convert(TempUnit::Kelvin).temp, 273.15);
		assert!(matches!(t.unit, TempUnit::Kelvin));

		let mut t = Temp::new(41.78, TempUnit::Celsius);
		assert_eq!(round_2_digits(t.convert(TempUnit::Kelvin).temp), 314.93);
		assert!(matches!(t.unit, TempUnit::Kelvin));
	}

	#[test]
	fn f_to_c() {
		let mut t = Temp::new(-40.0, TempUnit::Fahrenheit);
		assert_eq!(t.convert(TempUnit::Celsius).temp, -40.0);
		assert!(matches!(t.unit, TempUnit::Celsius));

		let mut t = Temp::new(63.5, TempUnit::Fahrenheit);
		assert_eq!(t.convert(TempUnit::Celsius).temp, 17.5);
		assert!(matches!(t.unit, TempUnit::Celsius));
	}

	#[test]
	fn f_to_k() {
		let mut t = Temp::new(-459.67, TempUnit::Fahrenheit);
		assert_eq!(t.convert(TempUnit::Kelvin).temp, 0.0);
		assert!(matches!(t.unit, TempUnit::Kelvin));

		let mut t = Temp::new(-31.72, TempUnit::Fahrenheit);
		assert_eq!(round_2_digits(t.convert(TempUnit::Kelvin).temp), 237.75);
		assert!(matches!(t.unit, TempUnit::Kelvin));
	}

	#[test]
	fn k_to_c() {
		let mut t = Temp::new(273.15, TempUnit::Kelvin);
		assert_eq!(t.convert(TempUnit::Celsius).temp, 0.0);
		assert!(matches!(t.unit, TempUnit::Celsius));

		let mut t = Temp::new(216.2, TempUnit::Kelvin);
		assert_eq!(round_2_digits(t.convert(TempUnit::Celsius).temp), -56.95);
		assert!(matches!(t.unit, TempUnit::Celsius));
	}

	#[test]
	fn k_to_f() {
		let mut t = Temp::new(0.0, TempUnit::Kelvin);
		assert_eq!(
			round_2_digits(t.convert(TempUnit::Fahrenheit).temp),
			-459.67
		);
		assert!(matches!(t.unit, TempUnit::Fahrenheit));

		let mut t = Temp::new(223.9, TempUnit::Kelvin);
		assert_eq!(round_2_digits(t.convert(TempUnit::Fahrenheit).temp), -56.65);
		assert!(matches!(t.unit, TempUnit::Fahrenheit));
	}

	#[test]
	fn c_to_c() {
		let mut t = Temp::new(0.0, TempUnit::Celsius);
		assert_eq!(t.convert(TempUnit::Celsius).value, 0.0);
		assert!(matches!(t.unit, TempUnit::Celsius));
	}

	#[test]
	fn f_to_f() {
		let mut t = Temp::new(165.4856, TempUnit::Fahrenheit);
		assert_eq!(t.convert(TempUnit::Fahrenheit).value, 165.4856);
		assert!(matches!(t.unit, TempUnit::Fahrenheit));
	}

	#[test]
	fn k_to_k() {
		let mut t = Temp::new(498.14, TempUnit::Kelvin);
		assert_eq!(t.convert(TempUnit::Kelvin).value, 498.14);
		assert!(matches!(t.unit, TempUnit::Kelvin));
	}

	fn round_2_digits(f: f64) -> f64 {
		(f * 100.0).round() / 100.0
	}
}
