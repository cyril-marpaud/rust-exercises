// mod ...

// use ...

pub struct Temp {
	temp: f64,
	unit: TempUnit,
}

pub enum TempUnit {
	Celsius,
	Farenheit,
	Kelvin,
}

impl Temp {
	fn compute(&self, to: &TempUnit) -> f64 {
		match (&self.unit, to) {
			(Celsius, Farenheit) => 9.0 * self.temp / 5.0 + 32.0,
			(Celsius, Kelvin) => self.temp + 273.15,
			(Farenheit, Celsius) => 5.0 * (self.temp - 32.0) / 9.0,
			(Farenheit, Kelvin) => 5.0 * (self.temp - 32.0) / 9.0 + 273.15,
			(Kelvin, Celsius) => self.temp - 273.15,
			(Kelvin, Farenheit) => 9.0 * (self.temp - 273.15) / 5.0 + 32.0,
			_ => self.temp,
		}
	}

	/// Convert a `Temp` into another unit.
	pub fn convert(&mut self, to: TempUnit) -> &mut Self {
		self.temp = self.compute(&to);
		self.unit = to;
		self
	}

	/// Create a new `Temp` object.
	pub fn new(temp: f64, unit: TempUnit) -> Self {
		Self { temp, unit }
	}
}

#[cfg(test)]
mod tests {
	use super::prelude::*;

	#[test]
	fn c_to_f() {
		let mut t = Temp::new(0.0, Celsius);
		assert_eq!(t.convert(Farenheit).temp, 32.0);
		assert!(matches!(t.unit, Farenheit));

		let mut t = Temp::new(50.5, Celsius);
		assert_eq!(t.convert(Farenheit).temp, 122.9);
		assert!(matches!(t.unit, Farenheit));
	}

	#[test]
	fn c_to_k() {
		let mut t = Temp::new(0.0, Celsius);
		assert_eq!(t.convert(Kelvin).temp, 273.15);
		assert!(matches!(t.unit, Kelvin));

		let mut t = Temp::new(41.78, Celsius);
		assert_eq!(round_2_digits(t.convert(Kelvin).temp), 314.93);
		assert!(matches!(t.unit, Kelvin));
	}

	#[test]
	fn f_to_c() {
		let mut t = Temp::new(-40.0, Farenheit);
		assert_eq!(t.convert(Celsius).temp, -40.0);
		assert!(matches!(t.unit, Celsius));

		let mut t = Temp::new(63.5, Farenheit);
		assert_eq!(t.convert(Celsius).temp, 17.5);
		assert!(matches!(t.unit, Celsius));
	}

	#[test]
	fn f_to_k() {
		let mut t = Temp::new(-459.67, Farenheit);
		assert_eq!(t.convert(Kelvin).temp, 0.0);
		assert!(matches!(t.unit, Kelvin));

		let mut t = Temp::new(-31.72, Farenheit);
		assert_eq!(round_2_digits(t.convert(Kelvin).temp), 237.75);
		assert!(matches!(t.unit, Kelvin));
	}

	#[test]
	fn k_to_c() {
		let mut t = Temp::new(0.0, Celsius);
		assert_eq!(t.convert(Kelvin).temp, 273.15);
		assert!(matches!(t.unit, Kelvin));

		let mut t = Temp::new(-56.95, Celsius);
		assert_eq!(t.convert(Kelvin).temp, 216.2);
		assert!(matches!(t.unit, Kelvin));
	}

	#[test]
	fn k_to_f() {
		let mut t = Temp::new(0.0, Kelvin);
		assert_eq!(round_2_digits(t.convert(Farenheit).temp), -459.67);
		assert!(matches!(t.unit, Farenheit));

		let mut t = Temp::new(223.9, Kelvin);
		assert_eq!(round_2_digits(t.convert(Farenheit).temp), -56.65);
		assert!(matches!(t.unit, Farenheit));
	}

	fn round_2_digits(f: f64) -> f64 {
		(f * 100.0).round() / 100.0
	}
}
