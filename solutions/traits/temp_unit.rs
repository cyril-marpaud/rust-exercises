use std::fmt::Display;

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
			"°{}",
			match self {
				TempUnit::Celsius => 'C',
				TempUnit::Fahrenheit => 'F',
				TempUnit::Kelvin => 'K',
			}
		)
	}
}
