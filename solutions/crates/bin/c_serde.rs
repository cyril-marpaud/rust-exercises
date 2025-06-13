use std::fs::File;

use rust_training::deserialize::Temp;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct City {
	#[serde(alias = "C")]
	name: String,
	#[serde(flatten)]
	temp: Temp,
}

fn main() {
	let file = File::open("data/temp_data.json").expect("Failed to open file");
	let cities: Vec<City> = serde_json::from_reader(file).expect("Failed to deserialize JSON");

	let min = cities.iter().min_by_key(|c| &c.temp).expect("Empty iter");
	println!("Min temp is {} in {}", min.temp, min.name);

	let max = cities.iter().max_by_key(|c| &c.temp).expect("Empty iter");
	println!("Max temp is {} in {}", max.temp, max.name);
}
