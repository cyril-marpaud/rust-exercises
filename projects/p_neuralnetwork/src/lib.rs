//! # TP : Réseau de Neurones - Conversion Celsius vers Fahrenheit
//!
//! ## Objectif
//!
//! Dans ce TP, vous allez implémenter un réseau de neurones simple capable d'apprendre
//! la conversion de températures Celsius vers Fahrenheit. La formule mathématique est :
//! F = C × 9/5 + 32
//!
//! Le réseau de neurones va apprendre cette relation linéaire sans qu'on lui fournisse
//! explicitement la formule.
//!
//! ## Architecture du réseau
//!
//! Pour cette tâche simple, nous utiliserons :
//! - 1 neurone d'entrée (température en Celsius)
//! - 2 neurones cachés (pour apprendre la transformation)
//! - 1 neurone de sortie (température en Fahrenheit)
//!
//! ## Étape 1 : Structure de base du réseau
//!
//! Commencez par définir les structures nécessaires pour représenter un réseau de neurones.
//!
//! ### 1.1 Définir la structure Layer
//!
//! Un layer (couche) contient :
//! - Les poids (weights) sous forme de matrice
//! - Les biais (biases) sous forme de vecteur
//! - La fonction d'activation
//!
//! <details>
//! <summary>💡 Indice : Structure Layer</summary>
//!
//! ```rust
//! struct Layer {
//!     weights: Vec<Vec<f64>>,
//!     biases: Vec<f64>,
//!     activation: ActivationFunction,
//! }
//! ```
//! </details>
//!
//! ### 1.2 Définir les fonctions d'activation
//!
//! Créez une énumération pour les fonctions d'activation. Pour ce TP, nous aurons besoin de :
//! - Linear : f(x) = x (pour la couche de sortie)
//! - ReLU : f(x) = max(0, x) (pour la couche cachée)
//!
//! <details>
//! <summary>💡 Indice : Implémentation des activations</summary>
//!
//! Pensez à implémenter aussi les dérivées pour la rétropropagation :
//! - Linear : f'(x) = 1
//! - ReLU : f'(x) = 1 si x > 0, sinon 0
//! </details>
//!
//! ### 1.3 Définir la structure NeuralNetwork
//!
//! Le réseau contient simplement un vecteur de layers.
//!
//! ## Étape 2 : Initialisation du réseau
//!
//! ### 2.1 Initialisation aléatoire des poids
//!
//! Implémentez une fonction pour initialiser les poids et biais de manière aléatoire.
//! Les poids doivent être initialisés avec de petites valeurs aléatoires (entre -0.5 et 0.5).
//!
//! Pour cette étape, ajoutez la dépendance `rand` à votre Cargo.toml :
//! ```toml
//! [dependencies]
//! rand = "0.8"
//! ```
//!
//! <details>
//! <summary>💡 Indice : Utilisation de rand</summary>
//!
//! ```rust
//! use rand::Rng;
//! let mut rng = rand::thread_rng();
//! let weight: f64 = rng.gen_range(-0.5..0.5);
//! ```
//! </details>
//!
//! ### 2.2 Constructeur du réseau
//!
//! Créez une fonction `new` qui prend en paramètre :
//! - Le nombre de neurones d'entrée
//! - Un vecteur contenant le nombre de neurones pour chaque couche cachée
//! - Le nombre de neurones de sortie
//!
//! ## Étape 3 : Propagation avant (Forward Pass)
//!
//! ### 3.1 Multiplication matrice-vecteur
//!
//! Implémentez une fonction pour multiplier une matrice par un vecteur.
//! Cette opération est au cœur de la propagation dans le réseau.
//!
//! <details>
//! <summary>💡 Indice : Multiplication matrice-vecteur</summary>
//!
//! Pour une matrice M et un vecteur v, le résultat r est :
//! ```
//! r[i] = Σ(M[i][j] * v[j]) pour tout j
//! ```
//! </details>
//!
//! ### 3.2 Propagation dans une couche
//!
//! Pour chaque couche :
//! 1. Multipliez l'entrée par les poids
//! 2. Ajoutez les biais
//! 3. Appliquez la fonction d'activation
//!
//! ### 3.3 Propagation dans tout le réseau
//!
//! Implémentez la méthode `forward` qui prend une entrée et retourne la sortie du réseau.
//! Gardez en mémoire toutes les sorties intermédiaires, elles seront nécessaires pour la rétropropagation.
//!
//! ## Étape 4 : Fonction de coût
//!
//! ### 4.1 Erreur quadratique moyenne (MSE)
//!
//! Implémentez la fonction de coût MSE :
//! MSE = (1/2) * (prédiction - cible)²
//!
//! ### 4.2 Dérivée de la fonction de coût
//!
//! La dérivée est : d(MSE)/d(prédiction) = prédiction - cible
//!
//! ## Étape 5 : Rétropropagation (Backpropagation)
//!
//! ### 5.1 Calcul des gradients
//!
//! Pour chaque couche en partant de la fin :
//! 1. Calculez le gradient de l'erreur par rapport aux sorties
//! 2. Multipliez par la dérivée de la fonction d'activation
//! 3. Calculez les gradients pour les poids et biais
//! 4. Propagez l'erreur vers la couche précédente
//!
//! <details>
//! <summary>💡 Indice : Formules de rétropropagation</summary>
//!
//! - Gradient des biais : δ (erreur de la couche)
//! - Gradient des poids : δ × entrée^T
//! - Erreur propagée : poids^T × δ
//! </details>
//!
//! ### 5.2 Mise à jour des poids
//!
//! Utilisez la descente de gradient :
//! - nouveau_poids = ancien_poids - learning_rate * gradient
//!
//! ## Étape 6 : Entraînement
//!
//! ### 6.1 Génération des données d'entraînement
//!
//! Créez un ensemble de données avec des températures Celsius de -40 à 40 (par pas de 5).
//! Calculez les températures Fahrenheit correspondantes.
//!
//! ### 6.2 Normalisation des données
//!
//! Normalisez les données d'entrée et de sortie entre 0 et 1 pour faciliter l'apprentissage.
//!
//! <details>
//! <summary>💡 Indice : Formule de normalisation</summary>
//!
//! ```
//! valeur_normalisée = (valeur - min) / (max - min)
//! ```
//! N'oubliez pas de sauvegarder min et max pour dénormaliser les prédictions !
//! </details>
//!
//! ### 6.3 Boucle d'entraînement
//!
//! Implémentez la boucle d'entraînement :
//! 1. Pour chaque époque
//! 2. Pour chaque exemple d'entraînement
//! 3. Forward pass
//! 4. Calcul de l'erreur
//! 5. Backpropagation
//! 6. Mise à jour des poids
//!
//! ## Étape 7 : Test et visualisation
//!
//! ### 7.1 Test du modèle
//!
//! Testez votre modèle sur de nouvelles températures non vues pendant l'entraînement.
//! Comparez avec la formule exacte.
//!
//! ### 7.2 Affichage des résultats
//!
//! Affichez :
//! - L'erreur moyenne sur l'ensemble de test
//! - Quelques exemples de prédictions
//! - L'évolution de l'erreur pendant l'entraînement
//!
//! ## Étape 8 : Améliorations (optionnel)
//!
//! ### 8.1 Sauvegarde et chargement du modèle
//!
//! Ajoutez la possibilité de sauvegarder les poids du réseau entraîné.
//! Pour cela, ajoutez la dépendance `serde` :
//! ```toml
//! [dependencies]
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```
//!
//! ### 8.2 Différentes architectures
//!
//! Expérimentez avec :
//! - Plus ou moins de neurones cachés
//! - Plus de couches
//! - Différentes fonctions d'activation (Sigmoid, Tanh)
//!
//! ### 8.3 Mini-batches
//!
//! Au lieu de mettre à jour les poids après chaque exemple, accumulez les gradients
//! sur plusieurs exemples avant la mise à jour.
//!
//! ## Points d'attention
//!
//! - Initialisez bien vos poids avec de petites valeurs aléatoires
//! - Utilisez un learning rate approprié (commencez avec 0.01)
//! - Normalisez vos données pour éviter les problèmes numériques
//! - Surveillez l'évolution de l'erreur pour détecter les problèmes
//!
//! ## Validation
//!
//! Votre réseau est correctement implémenté si :
//! - L'erreur diminue pendant l'entraînement
//! - Les prédictions sont proches des valeurs attendues (erreur < 1°F)
//! - Le réseau généralise bien sur de nouvelles températures

pub mod p_neuralnetwork;
