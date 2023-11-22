use std::fmt::Display;

#[derive(Default)]
pub enum TempUnit {
	#[default]
	Celsius,
	Farenheit,
	Kelvin,
}

impl Display for TempUnit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"°{}",
			match self {
				TempUnit::Celsius => 'C',
				TempUnit::Farenheit => 'F',
				TempUnit::Kelvin => 'K',
			}
		)
	}
}
