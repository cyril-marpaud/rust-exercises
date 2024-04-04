mod temp_unit;

use std::fmt::Display;

use temp_unit::TempUnit::{self, *};

#[derive(Debug, Default)]
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

impl Display for Temp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}{}", self.temp, self.unit)
	}
}

impl PartialEq for Temp {
	fn eq(&self, other: &Self) -> bool {
		self.temp.eq(&other.compute(&self.unit))
	}
}

impl PartialOrd for Temp {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.temp.partial_cmp(&other.compute(&self.unit))
	}
}
