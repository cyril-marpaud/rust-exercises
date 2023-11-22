mod prelude;
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
