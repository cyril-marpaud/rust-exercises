mod temp_unit;

use temp_unit::TempUnit::{self, *};

pub struct Temp {
	temp: f64,
	unit: TempUnit,
}

impl Temp {
	fn compute(&self, to: &TempUnit) -> f64 {
		match (&self.unit, to) {
			(Celsius, Fahrenheit) => 9.0 * self.temp / 5.0 + 32.0,
			(Celsius, Kelvin) => self.temp + 273.15,
			(Fahrenheit, Celsius) => 5.0 * (self.temp - 32.0) / 9.0,
			(Fahrenheit, Kelvin) => 5.0 * (self.temp - 32.0) / 9.0 + 273.15,
			(Kelvin, Celsius) => self.temp - 273.15,
			(Kelvin, Fahrenheit) => 9.0 * (self.temp - 273.15) / 5.0 + 32.0,
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
	let t2 = Temp::new(33.0, Fahrenheit);
	let t3 = Temp::new(273.15, Kelvin);

	assert!(t1 > t2);
	assert!(t1 > t3);
	assert!(t2 > t3);
}

#[test]
fn debug_c() {
	assert_eq!(
		format!("{:?}", Temp::new(12.86, Celsius)),
		"Temp { temp: 12.86, unit: Celsius }"
	);
	assert_eq!(format!("{:?}", Celsius), "Celsius");
}

#[test]
fn debug_f() {
	assert_eq!(
		format!("{:?}", Temp::new(57.24, Fahrenheit)),
		"Temp { temp: 57.24, unit: Fahrenheit }"
	);
	assert_eq!(format!("{:?}", Fahrenheit), "Fahrenheit");
}

#[test]
fn debug_k() {
	assert_eq!(
		format!("{:?}", Temp::new(96.84, Kelvin)),
		"Temp { temp: 96.84, unit: Kelvin }"
	);
	assert_eq!(format!("{:?}", Kelvin), "Kelvin");
}

#[test]
fn default_t() {
	assert_eq!(Temp::default(), Temp::new(0.0, Celsius));
}

#[test]
fn default_u() {
	assert!(matches!(TempUnit::default(), Celsius));
}

#[test]
fn display_c() {
	assert_eq!(format!("{}", Temp::new(54.84, Celsius)), "54.84°C");
	assert_eq!(format!("{}", Temp::new(-10.84, Celsius)), "-10.84°C");
	assert_eq!(format!("{}", Celsius), "°C");
}

#[test]
fn display_f() {
	assert_eq!(format!("{}", Temp::new(1337.0, Fahrenheit)), "1337°F");
	assert_eq!(format!("{}", Temp::new(-843.84, Fahrenheit)), "-843.84°F");
	assert_eq!(format!("{}", Fahrenheit), "°F");
}

#[test]
fn display_k() {
	assert_eq!(format!("{}", Temp::new(273.15, Kelvin)), "273.15K");
	assert_eq!(format!("{}", Temp::new(-34.2, Kelvin)), "-34.2K");
	assert_eq!(format!("{}", Kelvin), "K");
}

#[test]
fn equality() {
	let t1 = Temp::new(0.0, Celsius);
	let t2 = Temp::new(32.0, Fahrenheit);
	let t3 = Temp::new(273.15, Kelvin);

	assert_eq!(t1, t2);
	assert_eq!(t1, t3);
	assert_eq!(t2, t3);

	assert_eq!(
		Temp::new(50.5, TempUnit::Celsius),
		Temp::new(122.9, TempUnit::Fahrenheit)
	);

	assert_eq!(
		Temp::new(63.5, TempUnit::Fahrenheit),
		Temp::new(17.5, TempUnit::Celsius)
	);

	assert_eq!(
		Temp::new(-40.0, TempUnit::Fahrenheit),
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

#[test]
fn inequality() {
	let t1 = Temp::new(10.0, Celsius);
	let t2 = Temp::new(13.37, Fahrenheit);
	let t3 = Temp::new(420.0, Kelvin);

	assert_ne!(t1, t2);
	assert_ne!(t1, t3);
	assert_ne!(t2, t3);
}
