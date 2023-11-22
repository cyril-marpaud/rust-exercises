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
			(TempUnit::Celsius, TempUnit::Farenheit) => 9.0 * self.temp / 5.0 + 32.0,
			(TempUnit::Celsius, TempUnit::Kelvin) => self.temp + 273.15,
			(TempUnit::Farenheit, TempUnit::Celsius) => 5.0 * (self.temp - 32.0) / 9.0,
			(TempUnit::Farenheit, TempUnit::Kelvin) => 5.0 * (self.temp - 32.0) / 9.0 + 273.15,
			(TempUnit::Kelvin, TempUnit::Celsius) => self.temp - 273.15,
			(TempUnit::Kelvin, TempUnit::Farenheit) => 9.0 * (self.temp - 273.15) / 5.0 + 32.0,
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
