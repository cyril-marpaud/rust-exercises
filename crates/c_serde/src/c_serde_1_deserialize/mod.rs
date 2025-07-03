use std::fmt::Display;

use TempUnit::*;

#[derive(Debug, Default)]
pub enum TempUnit {
	#[default]
	Celsius,
	Fahrenheit,
	Kelvin,
}

impl Display for TempUnit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				TempUnit::Celsius => "°C",
				TempUnit::Fahrenheit => "°F",
				TempUnit::Kelvin => "K",
			}
		)
	}
}

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

#[cfg(test)]
mod tests {
	const COLDEST: &str = "";
	const HOTTEST: &str = "";

	#[test]
	fn coldest() {
		assert_eq!(
			format!("{}", blake3::hash(COLDEST.to_ascii_lowercase().as_bytes())),
			"e08ecf451f7a371a45caa11c1d007f408d2d123648955d7260a430c2131dc005"
		);
	}

	#[test]
	fn hottest() {
		assert_eq!(
			format!("{}", blake3::hash(HOTTEST.to_ascii_lowercase().as_bytes())),
			"32183683c8972d26dcefb6d13d283bde009616bb1029e865428dc38eceb6ebb5"
		);
	}
}
