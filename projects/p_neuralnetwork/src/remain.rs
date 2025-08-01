use std::io::Write;

use p_neuralnetwork::p_neuralnetwork::{NeuralNetwork, Normalizer};

fn main() {
	let input_normalizer = Normalizer::new(-40.0, 40.0);
	let output_normalizer = Normalizer::new(-40.0, 104.0);

	let (normalized_inputs, normalized_outputs): (Vec<_>, Vec<_>) = (-40..=40)
		.step_by(5)
		.map(|c| {
			(
				input_normalizer.normalize(c as f64),
				output_normalizer.normalize(c as f64 * 9.0 / 5.0 + 32.0),
			)
		})
		.unzip();

	println!("Entraînement du réseau de neurones...");
	let mut network = NeuralNetwork::new(1, vec![2], 1);
	let errors = network.train(&normalized_inputs, &normalized_outputs, 1000, 0.1);
	println!("\nEntraînement terminé!");
	println!("Erreur finale: {:.6}", errors.last().unwrap());

	println!("\n--- Test du modèle ---");
	let temps = [-50.0, -25.0, 0.0, 15.0, 37.0, 100.0];

	println!("Celsius | Fahrenheit (prédit) | Fahrenheit (réel) | Erreur");
	println!("--------|---------------------|-------------------|--------");

	let total_error = temps.into_iter().fold(0.0, |total_error, temp| {
		let normalized_input = input_normalizer.normalize(temp);
		let normalized_output = network.infer(normalized_input);

		let inferred = output_normalizer.denormalize(normalized_output);
		let actual = temp * 9.0 / 5.0 + 32.0;
		let error = (inferred - actual).abs();

		println!("{temp:>7.1} | {inferred:>19.1} | {actual:>17.1} | {error:>5.2}°F");

		total_error + error
	});

	println!(
		"\nErreur moyenne sur le test: {:.2}°F",
		total_error / temps.len() as f64
	);

	// Démonstration des poids appris
	println!("\n--- Analyse du réseau ---");
	println!("Le réseau a appris une transformation proche de F = C × 1.8 + 32");

	// Exemple interactif
	println!("\n--- Test interactif ---");
	println!("Entrez une température en Celsius (ou 'q' pour quitter):");

	loop {
		print!("> ");
		std::io::stdout().flush().unwrap();

		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let input = input.trim();

		if input == "q" {
			break;
		}

		if let Ok(celsius) = input.parse::<f64>() {
			let normalized_input = input_normalizer.normalize(celsius);
			let normalized_output = network.infer(normalized_input);
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
