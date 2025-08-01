enum ActivationFunction {
	Linear,
	ReLU,
}

impl ActivationFunction {
	fn apply(&self, x: f64) -> f64 {
		match self {
			ActivationFunction::Linear => x,
			ActivationFunction::ReLU => x.max(0.0),
		}
	}

	fn derivative(&self, x: f64) -> f64 {
		match self {
			ActivationFunction::Linear => 1.0,
			ActivationFunction::ReLU => {
				if x > 0.0 {
					1.0
				} else {
					0.0
				}
			}
		}
	}
}

struct Layer {
	weights: Vec<Vec<f64>>,
	biases: Vec<f64>,
	activation: ActivationFunction,
}

impl Layer {
	fn new(input_size: usize, output_size: usize, activation: ActivationFunction) -> Self {
		let weights = (0..output_size)
			.map(|_| {
				(0..input_size)
					.map(|_| rand::random_range(-0.5..0.5))
					.collect()
			})
			.collect();

		let biases = (0..output_size)
			.map(|_| rand::random_range(-0.5..0.5))
			.collect();

		Layer {
			weights,
			biases,
			activation,
		}
	}

	fn forward(&self, input: &[f64]) -> Vec<f64> {
		let mut output = vec![0.0; self.biases.len()];

		for (i, (weight_row, bias)) in self.weights.iter().zip(&self.biases).enumerate() {
			output[i] = weight_row
				.iter()
				.zip(input)
				.map(|(w, x)| w * x)
				.sum::<f64>()
				+ bias;
			output[i] = self.activation.apply(output[i]);
		}

		output
	}
}

pub struct NeuralNetwork {
	layers: Vec<Layer>,
}

impl NeuralNetwork {
	pub fn new(input_size: usize, hidden_sizes: Vec<usize>, output_size: usize) -> Self {
		let mut layers = Vec::new();
		let mut current_size = input_size;

		// Couches cachées
		for &hidden_size in hidden_sizes.iter() {
			let activation = ActivationFunction::ReLU;
			layers.push(Layer::new(current_size, hidden_size, activation));
			current_size = hidden_size;
		}

		// Couche de sortie
		layers.push(Layer::new(
			current_size,
			output_size,
			ActivationFunction::Linear,
		));

		NeuralNetwork { layers }
	}

	fn forward(&self, input: &[f64]) -> Vec<Vec<f64>> {
		let mut activations = vec![input.to_vec()];
		let mut current = input.to_vec();

		for layer in &self.layers {
			current = layer.forward(&current);
			activations.push(current.clone());
		}

		activations
	}

	fn backward(&mut self, target: f64, activations: &[Vec<f64>], learning_rate: f64) {
		let mut error = activations.last().unwrap()[0] - target; // Dérivée MSE

		// Rétropropagation à travers les couches
		for (layer_idx, layer) in self.layers.iter_mut().enumerate().rev() {
			let layer_inputs = &activations[layer_idx];
			let layer_outputs = &activations[layer_idx + 1];

			// Calcul des gradients
			let mut next_error = vec![0.0; layer_inputs.len()];

			for (i, (weight_row, bias)) in
				layer.weights.iter_mut().zip(&mut layer.biases).enumerate()
			{
				// Applique la dérivée de la fonction d'activation
				let delta = error * layer.activation.derivative(layer_outputs[i]);

				// Met à jour les biais
				*bias -= learning_rate * delta;

				// Met à jour les poids et calcule l'erreur pour la couche précédente
				for (j, weight) in weight_row.iter_mut().enumerate() {
					next_error[j] += *weight * delta;
					*weight -= learning_rate * delta * layer_inputs[j];
				}
			}

			error = if layer_idx == 0 { 0.0 } else { next_error[0] };
		}
	}

	pub fn train(
		&mut self,
		inputs: &[f64],
		targets: &[f64],
		epochs: usize,
		learning_rate: f64,
	) -> Vec<f64> {
		let mut errors = Vec::new();

		for epoch in 0..epochs {
			let mut total_error = 0.0;

			for (input, target) in inputs.iter().zip(targets) {
				// Forward pass
				let activations = self.forward(&[*input]);

				// Calcul de l'erreur
				let error = 0.5 * (activations.last().unwrap()[0] - target).powi(2);
				total_error += error;

				// Backward pass
				self.backward(*target, &activations, learning_rate);
			}

			let avg_error = total_error / inputs.len() as f64;
			errors.push(avg_error);

			if epoch % 100 == 0 {
				println!("Époque {epoch}: Erreur moyenne = {avg_error:.6}");
			}
		}

		errors
	}

	pub fn infer(&self, input: f64) -> f64 {
		let activations = self.forward(&[input]);
		activations.last().unwrap()[0]
	}
}

// Fonctions de normalisation/dénormalisation
pub struct Normalizer {
	min: f64,
	max: f64,
}

impl Normalizer {
	pub fn new(min: f64, max: f64) -> Self {
		Normalizer { min, max }
	}

	pub fn normalize(&self, value: f64) -> f64 {
		(value - self.min) / (self.max - self.min)
	}

	pub fn denormalize(&self, value: f64) -> f64 {
		value * (self.max - self.min) + self.min
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_activation_functions() {
		assert_eq!(ActivationFunction::Linear.apply(5.0), 5.0);
		assert_eq!(ActivationFunction::Linear.derivative(5.0), 1.0);

		assert_eq!(ActivationFunction::ReLU.apply(5.0), 5.0);
		assert_eq!(ActivationFunction::ReLU.apply(-5.0), 0.0);
		assert_eq!(ActivationFunction::ReLU.derivative(5.0), 1.0);
		assert_eq!(ActivationFunction::ReLU.derivative(-5.0), 0.0);
	}

	#[test]
	fn test_normalizer() {
		let norm = Normalizer::new(0.0, 100.0);
		assert_eq!(norm.normalize(0.0), 0.0);
		assert_eq!(norm.normalize(50.0), 0.5);
		assert_eq!(norm.normalize(100.0), 1.0);

		assert_eq!(norm.denormalize(0.0), 0.0);
		assert_eq!(norm.denormalize(0.5), 50.0);
		assert_eq!(norm.denormalize(1.0), 100.0);
	}

	#[test]
	fn test_network_structure() {
		let network = NeuralNetwork::new(1, vec![2], 1);
		assert_eq!(network.layers.len(), 2);
		assert_eq!(network.layers[0].weights.len(), 2); // 2 neurones cachés
		assert_eq!(network.layers[1].weights.len(), 1); // 1 neurone de sortie
	}
}
