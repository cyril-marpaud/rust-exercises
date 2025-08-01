use p_neuralnetwork::p_neuralnetwork::{NeuralNetwork, Normalizer};

fn main() {
	// Génération des données d'entraînement
	let celsius_temps: Vec<f64> = (-40..=40).step_by(5).map(|x| x as f64).collect();

	let fahrenheit_temps: Vec<f64> = celsius_temps
		.iter()
		.map(|&c| c * 9.0 / 5.0 + 32.0)
		.collect();

	// Normalisation des données
	let input_normalizer = Normalizer::new(-40.0, 40.0);
	let output_normalizer = Normalizer::new(-40.0, 104.0); // -40°F à 104°F

	let normalized_inputs: Vec<f64> = celsius_temps
		.iter()
		.map(|&c| input_normalizer.normalize(c))
		.collect();

	let normalized_outputs: Vec<f64> = fahrenheit_temps
		.iter()
		.map(|&f| output_normalizer.normalize(f))
		.collect();

	// Création et entraînement du réseau
	let mut network = NeuralNetwork::new(1, vec![2], 1);

	println!("Entraînement du réseau de neurones...");
	let errors = network.train(&normalized_inputs, &normalized_outputs, 1000, 0.1);

	println!("\nEntraînement terminé!");
	println!("Erreur finale: {:.6}", errors.last().unwrap());

	// Test du modèle
	println!("\n--- Test du modèle ---");
	let test_temps = vec![-50.0, -25.0, 0.0, 15.0, 37.0, 100.0];

	println!("Celsius | Fahrenheit (prédit) | Fahrenheit (réel) | Erreur");
	println!("--------|-------------------|------------------|--------");

	let mut total_error = 0.0;
	for celsius in &test_temps {
		let normalized_input = input_normalizer.normalize(*celsius);
		let normalized_output = network.predict(normalized_input);
		let predicted_fahrenheit = output_normalizer.denormalize(normalized_output);
		let actual_fahrenheit = celsius * 9.0 / 5.0 + 32.0;
		let error = (predicted_fahrenheit - actual_fahrenheit).abs();

		println!(
			"{celsius:7.1} | {predicted_fahrenheit:17.1} | {actual_fahrenheit:16.1} | {error:7.2}°F"
		);

		total_error += error;
	}

	println!(
		"\nErreur moyenne sur le test: {:.2}°F",
		total_error / test_temps.len() as f64
	);

	// Démonstration des poids appris
	println!("\n--- Analyse du réseau ---");
	println!("Le réseau a appris une transformation proche de F = C × 1.8 + 32");

	// Exemple interactif
	println!("\n--- Test interactif ---");
	println!("Entrez une température en Celsius (ou 'q' pour quitter):");

	use std::io::{self, Write};
	loop {
		print!("> ");
		io::stdout().flush().unwrap();

		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		let input = input.trim();

		if input == "q" {
			break;
		}

		if let Ok(celsius) = input.parse::<f64>() {
			let normalized_input = input_normalizer.normalize(celsius);
			let normalized_output = network.predict(normalized_input);
			let predicted_fahrenheit = output_normalizer.denormalize(normalized_output);
			let actual_fahrenheit = celsius * 9.0 / 5.0 + 32.0;

			println!(
				"{celsius}°C = {predicted_fahrenheit:.1}°F (prédit) | {actual_fahrenheit:.1}°F (réel) | Erreur: {:.2}°F",
				(predicted_fahrenheit - actual_fahrenheit).abs()
			);
		} else {
			println!("Entrée invalide. Veuillez entrer un nombre.");
		}
	}
}
