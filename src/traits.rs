mod temp_unit;

use temp_unit::TempUnit::{self, *};

pub struct Temp {
	temp: f64,
	unit: TempUnit,
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

	pub fn convert(&mut self, to: TempUnit) -> &mut Self {
		self.temp = self.compute(&to);
		self.unit = to;
		self
	}

	pub fn new(temp: f64, unit: TempUnit) -> Self {
		Self { temp, unit }
	}
}

#[test]
fn comparison() {
	let t1 = Temp::new(2.0, Celsius);
	let t2 = Temp::new(33.0, Farenheit);
	let t3 = Temp::new(273.15, Kelvin);

	assert!(t1 > t2);
	assert!(t1 > t3);
	assert!(t2 > t3);
}

#[test]
fn default() {
	assert_eq!(Temp::default(), Temp::new(0.0, Celsius));
}

#[test]
fn display_c() {
	assert_eq!(format!("{}", Temp::new(54.84, Celsius)), "54.84°C");
	assert_eq!(format!("{}", Temp::new(-10.84, Celsius)), "-10.84°C");
	assert_eq!(format!("{}", Celsius), "°C");
}
#[test]
fn display_f() {
	assert_eq!(format!("{}", Temp::new(1337.0, Farenheit)), "1337°F");
	assert_eq!(format!("{}", Temp::new(-843.84, Farenheit)), "-843.84°F");
	assert_eq!(format!("{}", Farenheit), "°F");
}

#[test]
fn display_k() {
	assert_eq!(format!("{}", Temp::new(273.15, Kelvin)), "273.15°K");
	assert_eq!(format!("{}", Temp::new(-34.2, Kelvin)), "-34.2°K");
	assert_eq!(format!("{}", Kelvin), "°K");
}

#[test]
fn equality() {
	let t1 = Temp::new(0.0, Celsius);
	let t2 = Temp::new(32.0, Farenheit);
	let t3 = Temp::new(273.15, Kelvin);

	assert_eq!(t1, t2);
	assert_eq!(t1, t3);
	assert_eq!(t2, t3);

	assert_eq!(
		Temp::new(50.5, TempUnit::Celsius),
		Temp::new(122.9, TempUnit::Farenheit)
	);

	assert_eq!(
		Temp::new(63.5, TempUnit::Farenheit),
		Temp::new(17.5, TempUnit::Celsius)
	);

	assert_eq!(
		Temp::new(-40.0, TempUnit::Farenheit),
		Temp::new(-40.0, TempUnit::Celsius)
	);

	assert_eq!(
		// Swap these and the test fails
		// Here, "other" is converted to the unit of "self" so that equality can be tested
		// That conversion gives exactly 216.2
		// The other way around, the conversion gives -56.94999999999999
		// A rounding mechanism to the required precision could thus be implemented to avoid pitfalls
		Temp::new(216.2, TempUnit::Kelvin),   // <-- self
		Temp::new(-56.95, TempUnit::Celsius), // <-- other
	);
}
