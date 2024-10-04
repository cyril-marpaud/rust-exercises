use std::fmt::Display;

use serde::Deserialize;

#[derive(Default, Deserialize)]
pub enum TempUnit {
	#[default]
	#[serde(alias = "C")]
	Celsius,
	#[serde(alias = "F")]
	Fahrenheit,
	#[serde(alias = "K")]
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
