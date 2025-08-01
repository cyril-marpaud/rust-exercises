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
	let temps = [
		-100.0, -50.0, -40.0, -30.0, -25.0, -20.0, -15.0, -10.0, -5.0, 0.0, 5.0, 10.0, 15.0, 20.0,
		25.0, 30.0, 40.0, 50.0, 100.0,
	];

	println!("Celsius | Fahrenheit (prédit) | Fahrenheit (réel) | Erreur");
	println!("--------|---------------------|-------------------|--------");

	temps.into_iter().for_each(|temp| {
		let normalized_input = input_normalizer.normalize(temp);
		let normalized_output = network.infer(normalized_input);
		let inferred = output_normalizer.denormalize(normalized_output);

		let actual = temp * 9.0 / 5.0 + 32.0;
		let error = (inferred - actual).abs();

		println!("{temp:>7.1} | {inferred:>19.1} | {actual:>17.1} | {error:>5.2}°F");
	});
}
