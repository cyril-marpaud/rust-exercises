//! # TP : Réseau de Neurones - Conversion Celsius vers Fahrenheit
//!
//! ## Objectif
//!
//! Dans ce TP, vous allez implémenter un réseau de neurones simple capable d'apprendre la conversion de températures Celsius vers Fahrenheit. La formule mathématique est $F = C \times \frac{9}{5} + 32$.
//!
//! Le réseau de neurones va apprendre cette relation affine sans qu'on lui fournisse explicitement la formule. Il ne "comprend" pas les températures, il ajuste juste des nombres jusqu'à ce que ses prédictions correspondent aux exemples.
//!
//! ## Introduction : Qu'est-ce qu'un neurone artificiel ?
//!
//! Un neurone artificiel est inspiré du neurone biologique. Il reçoit des entrées, les traite, et produit une sortie.
//!
//! ```
//! Entrées            Neurone          Sortie
//!    ↓
//!    x₁ ───w₁──┐ somme w⨉x   activation
//!              │     ↓           ↓
//!    x₂ ───w₂──┼───[ Σ + b ]───[ f ]───> y
//!              │         ↑
//!    x₃ ───w₃──┘       biais
//!          ↑
//!        Poids
//! ```
//!
//! ### Les composants :
//!
//! - **Entrées (x)** : Les valeurs qu'on donne au neurone (ex: température en Celsius)
//! - **Poids (w)** : L'importance de chaque entrée (comme un volume sur un canal audio)
//! - **Biais (b)** : Un ajustement constant (comme le réglage de luminosité de base d'un écran)
//! - **Fonction d'activation (f)** : Transformation finale (ex: ReLU = "si négatif, mets 0")
//!
//! ### Calcul de la sortie du neurone :
//!
//! $$ y = f(x_1 \times w_1 + x_2 \times w_2 + x_3 \times w_3 + b) $$
//!
//! ## Architecture du réseau
//!
//! Pour cette tâche simple, nous utiliserons :
//! - 1 neurone d'entrée (température en Celsius)
//! - 2 neurones cachés (pour apprendre la transformation)
//! - 1 neurone de sortie (température en Fahrenheit)
//!
//! ```
//! ENTRÉE               COUCHE CACHÉE           SORTIE
//!   (1)                    (2)                  (1)
//!
//!            ┌─── w₁₁ ───> [N₁] ─── w₂₁ ───┐
//!   20°C ────┤                             ├───> 68°F
//!            └─── w₁₂ ───> [N₂] ─── w₂₂ ───┘
//! ```
//!
//! ## Étape 1 : Structure de base du réseau
//!
//! ### Comprendre les poids et biais : Pourquoi des matrices ?
//!
//! #### Pour UN neurone :
//!
//! ```
//! Température ────[Neurone: w=1.8, b=32]────> Contribution
//!     20°C            20×1.8+32=68                 68°F
//! ```
//!
//! #### Pour PLUSIEURS neurones (une couche) :
//!
//! ```
//!           ┌───[Neurone₁: w=0.9, b=10]──> 28°
//! 20°C ─────┤         20×0.9+10=28
//!           └───[Neurone₂: w=2.5, b=5]───> 55°
//!                     20×2.5+5=55
//! ```
//!
//! Chaque neurone a **son propre poids** pour chaque entrée !
//!
//! #### Représentation matricielle :
//!
//! Pour une couche de 2 neurones recevant 1 entrée :
//!
//! $$
//! W = \begin{bmatrix} w_1 \\\\ w_2 \end{bmatrix} = \begin{bmatrix} 0.9 \\\\ 2.5 \end{bmatrix}
//! \quad \text{et} \quad
//! B = \begin{bmatrix} b_1 \\\\ b_2 \end{bmatrix} = \begin{bmatrix} 10 \\\\ 5 \end{bmatrix}
//! $$
//!
//! **Pourquoi une matrice ?** Parce qu'avec plusieurs entrées et plusieurs neurones :
//!
//! $$
//! W = \begin{bmatrix}
//! w_{11} & w_{12} \\\\
//! w_{21} & w_{22} \\\\
//! w_{31} & w_{32}
//! \end{bmatrix}
//! \quad \text{(2 entrées → 3 neurones = 6 connexions = matrice } 3 \times 2\text{)}
//! $$
//!
//! ### 1.1 Définir la structure `Layer`
//!
//! Implémentez une structure `Layer` qui représente une couche de neurones. Pour les poids et les biais, nous utiliserons le type `f64`.
//!
//! Une layer (couche) se compose :
//! - Des poids sous forme de matrice
//! - Des biais sous forme de vecteur
//! - D'une fonction d'activation : un enum `ActivationFunction` dont nous définirons les variants dans la prochaine section
//!
//! <details>
//! <summary>💡 Indice : matrices</summary>
//!
//! Une matrice peut être représentée sous forme d'un vecteur de vecteurs.
//! </details>
//!
//! ### 1.2 Définir les fonctions d'activation
//!
//! Les fonctions d'activation permettent d'introduire de la non-linéarité dans le réseau. Il existe de nombreuses fonctions d'activation possibles (Sigmoid, Tanh, Leaky ReLU, ELU, etc.), mais nous allons utiliser les deux plus simples :
//!
//! - **Linear** : $f(x) = x$ (pour la couche de sortie car on veut une valeur non bornée)
//! - **ReLU** : $f(x) = \max(0, x)$ (pour la couche cachée)
//!
//! ReLU est privilégiée pour les couches cachées car elle est très simple à calculer tout en évitant les problèmes de gradient qui disparaît, contrairement aux fonctions saturantes comme Sigmoid ou Tanh.
//!
//! Implémenter :
//! - Un enum `ActivationFunction` avec les variants `Linear` et `ReLU`
//! - La méthode `apply` pour appliquer la fonction d'activation sur un `f64`
//! - La méthode `derivative` pour calculer la dérivée (nécessaire pour la rétropropagation) :
//!   - **Linear** : $f'(x) = 1$
//!   - **ReLU** : $f'(x) = \begin{cases} 1 & \text{si } x > 0 \\\\ 0 & \text{sinon} \end{cases}$
//!
//! ### 1.3 Définir la structure `NeuralNetwork`
//!
//! Implémenter une structure `NeuralNetwork` qui représente le réseau de neurones complet.
//!
//! Le réseau est simplement composé d'un vecteur de layers.
//!
//! ## Étape 2 : Initialisation du réseau
//!
//! ### Pourquoi initialiser aléatoirement ?
//!
//! Si tous les poids et biais sont initialisés à 0, tous les neurones d'une même couche produisent la même sortie et reçoivent les mêmes gradients pendant l'entraînement. Ils évoluent donc de manière identique et le réseau ne peut pas apprendre de représentations complexes. L'initialisation aléatoire brise cette symétrie.
//!
//! **Attention** : Des poids trop grands peuvent causer des problèmes :
//! - Valeurs qui explosent
//! - Neurones morts avec ReLU (si entrée × poids + biais < 0 → toujours 0)
//!
//! C'est pourquoi nous initialiserons les poids et biais avec de petites valeurs aléatoires entre -0.5 et 0.5.
//!
//! ### 2.1 Constructeur de Layer
//!
//! Implémenter un constructeur `new` pour la structure `Layer` qui :
//! - Prend en paramètre :
//!   - Le nombre de neurones d'entrée
//!   - Le nombre de neurones de sortie
//!   - La fonction d'activation à utiliser
//! - Crée et initialise aléatoirement la matrice des poids
//! - Crée et initialise aléatoirement le vecteur des biais
//!
//! <details>
//! <summary>💡 Indice : initialisation aléatoire</summary>
//!
//! On peut utiliser la méthode [`gen_range`](https://docs.rs/rand/latest/rand/trait.Rng.html#method.gen_range) du crate [`rand`](https://docs.rs/rand/).
//! </details>
//!
//! ### 2.2 Constructeur de NeuralNetwork
//!
//! Implémenter un constructeur `new` pour la structure `NeuralNetwork` qui :
//! - Prend en paramètre :
//!   - Le nombre de neurones d'entrée
//!   - Un vecteur contenant le nombre de neurones pour chaque couche cachée
//!   - Le nombre de neurones de sortie
//! - Construit automatiquement toutes les couches nécessaires :
//!   - Les couches cachées utilisent la fonction d'activation ReLU
//!   - La couche de sortie utilise la fonction d'activation Linear
//! - Retourne le réseau initialisé
//!
//! <details>
//! <summary>💡 Indice : Architecture pour notre problème</summary>
//!
//! Pour la conversion Celsius → Fahrenheit, nous allons créer un réseau avec :
//! - 1 neurone d'entrée (la température en Celsius)
//! - 2 neurones dans la couche cachée
//! - 1 neurone de sortie (la température en Fahrenheit)
//!
//! Donc l'appel au constructeur ressemblera à : `NeuralNetwork::new(1, vec![2], 1)`
//! </details>
//!
//! ## Étape 3 : Normalisation des données
//!
//! ### Pourquoi normaliser ?
//!
//! Les réseaux de neurones fonctionnent mieux avec des valeurs entre 0 et 1 :
//! - **Problèmes numériques** : Des valeurs comme 40°C peuvent, après multiplication par les poids et accumulation dans les couches, produire des nombres très grands causant des overflows
//! - **Gradients déséquilibrés** : Sans normalisation, les gradients pour les grandes valeurs dominent l'apprentissage
//! - **Convergence instable** : Les poids doivent compenser les différentes échelles, ralentissant l'apprentissage
//!
//! ### Processus de normalisation
//!
//! La normalisation transforme une plage de valeurs [min, max] vers [0, 1] :
//!
//! ```
//! Valeurs originales :  -40°C ────────── 0°C ────────── 40°C
//!                         ↓               ↓               ↓
//! Valeurs normalisées :   0 ──────────── 0.5 ──────────── 1
//! ```
//!
//! Après l'entraînement, il faudra dénormaliser les prédictions pour obtenir des températures réelles :
//!
//! ```
//! Entrée → Normalisation → Réseau → Sortie normalisée → Dénormalisation → Résultat
//! 20°C   →     0.75      →   NN   →       0.85        →      68°F       →   68°F
//! ```
//!
//! ### 3.1 Structure de normalisation
//!
//! Implémenter une structure `Normalizer` qui :
//! - Contient deux champs `f64` : `min` et `max` définissant la plage de valeurs
//! - Possède un constructeur `new` prenant ces deux valeurs en paramètres
//! - Implémente deux méthodes associées :
//!   - `normalize` : $ x_{norm} = \frac{x - x_{min}}{x_{max} - x_{min}} $
//!   - `denormalize` : $ x = x_{norm} \times (x_{max} - x_{min}) + x_{min} $
//!
//! ### 3.2 Normaliseurs pour notre problème
//!
//! Nous aurons besoin de deux instances de `Normalizer` :
//! - Une pour les températures Celsius (entrée) : plage [-40, 40]
//! - Une pour les températures Fahrenheit (sortie) : plage [-40, 104]
//!
//! Durant l'entraînement, il ne faudra pas oublier de :
//! 1. Normaliser la température Celsius avant de la donner au réseau
//! 2. Normaliser la température Fahrenheit cible pour calculer l'erreur
//! 3. Dénormaliser la sortie après prédiction pour obtenir la température en Fahrenheit
//!
//! ## Étape 4 : Propagation avant (Forward Pass)
//!
//! ### Comprendre le Forward Pass
//!
//! Le forward pass, c'est suivre le chemin des données de l'entrée vers la sortie.
//!
//! ```
//! ENTRÉE               COUCHE CACHÉE           SORTIE
//!   (1)                    (2)                  (1)
//!
//!            ┌─── w₁₁ ───> [N₁] ─── w₂₁ ───┐
//!   20°C ────┤                             ├───> 68°F
//!            └─── w₁₂ ───> [N₂] ─── w₂₂ ───┘
//!                                        
//! Étape 1: Calcul couche cachée    Étape 2: Calcul sortie
//! N₁ = ReLU(20×w₁₁ + b₁)           Sortie = Linear(N₁×w₂₁ + N₂×w₂₂ + b₃)
//! N₂ = ReLU(20×w₁₂ + b₂)
//! ```
//!
//! ### Exemple concret pas à pas :
//!
//! 1. **Entrée** : 20°C (normalisé à 0.75)
//! 2. **Vers neurone caché 1** : 0.75 × 0.3 + 0.1 = 0.325 → ReLU → 0.325
//! 3. **Vers neurone caché 2** : 0.75 × -0.2 + 0.4 = 0.25 → ReLU → 0.25  
//! 4. **Vers sortie** : 0.325 × 2.0 + 0.25 × 1.5 + 0.2 = 1.225
//! 5. **Dénormalisation** : 68°F
//!
//! ### 4.1 Forward pass pour une couche (Layer)
//!
//! Nous allons commencer par implémenter la propagation avant pour une seule couche. Chaque couche transforme son entrée selon la formule mathématique du neurone.
//!
//! Implémenter une méthode `forward` sur `Layer` qui :
//! - Prend en paramètre une slice d'entrée (`&[f64]`) représentant les valeurs d'entrée de la couche
//! - Retourne un vecteur (`Vec<f64>`) contenant les sorties de tous les neurones de cette couche
//!
//! #### Pourquoi une slice en entrée ?
//!
//! Même si notre problème n'a qu'une seule valeur d'entrée (température), les couches cachées reçoivent les sorties de la couche précédente qui peuvent être multiples (2 neurones cachés dans notre cas).
//!
//! #### Algorithme détaillé pour chaque neurone :
//!
//! Pour une couche avec `n` neurones de sortie, créer un vecteur `output` de taille `n`. Pour chaque neurone `i` :
//!
//! 1. **Calculer la somme pondérée** :
//!    - Parcourir `weights[i]` et `input` simultanément
//!    - Multiplier chaque poids par son entrée correspondante
//!    - Faire la somme de tous ces produits
//!
//! 2. **Ajouter le biais** :
//!    - Ajouter `biases[i]` à la somme calculée
//!
//! 3. **Appliquer l'activation** :
//!    - Utiliser la méthode `apply` de `self.activation` sur le résultat
//!
//! <details>
//! <summary>💡 Indice : Itération simultanée</summary>
//!
//! Pour parcourir deux vecteurs simultanément en Rust, utiliser la méthode [`zip`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip) :
//!
//! ```rust
//! weights[i].iter().zip(input)
//! ```
//!
//! Pour calculer une somme de produits, chaîner avec les méthodes [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) et [`sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum).
//! </details>
//!
//! ### 4.2 Forward pass pour le réseau complet
//!
//! Le réseau doit propager l'entrée à travers toutes les couches successivement.
//!
//! #### Deux besoins différents
//!
//! Notre réseau aura besoin de deux méthodes de forward pass :
//!
//! 1. **Pour l'entraînement** : Garder toutes les activations intermédiaires pour la rétropropagation
//! 2. **Pour l'inférence** : Juste obtenir la sortie finale
//!
//! #### 4.2.1 Forward pass complet (pour l'entraînement)
//!
//! Implémenter une méthode `forward` sur `NeuralNetwork` qui :
//! - Prend une slice `&[f64]` en entrée (même si c'est un seul élément pour notre problème)
//! - Retourne `Vec<Vec<f64>>` contenant TOUTES les activations (entrée + sorties de chaque couche)
//!
//! **Pourquoi garder toutes les activations ?**
//!
//! Lors de la rétropropagation, pour calculer les gradients d'une couche, nous avons besoin :
//! - Des activations de la couche précédente (entrées de cette couche)
//! - Des activations de cette couche (pour appliquer la dérivée de l'activation)
//!
//! **Algorithme :**
//!
//! 1. Créer un vecteur `activations` et y ajouter l'entrée
//! 2. Initialiser `current` avec l'entrée
//! 3. Pour chaque couche :
//!    - Calculer `current = layer.forward(&current)`
//!    - Ajouter `current` au vecteur d'`activations`
//! 4. Retourner `activations`
//!
//! <details>
//! <summary>💡 Indice : Structure des activations</summary>
//!
//! Pour un réseau [1, 2, 1] avec entrée 0.75, les activations ressembleraient à :
//! ```
//! [
//!   [0.75],        // Entrée
//!   [0.3, 0.5],    // Sortie couche cachée (2 neurones)
//!   [0.85]         // Sortie finale
//! ]
//! ```
//! </details>
//!
//! #### 4.2.2 Forward pass simple (pour l'inférence)
//!
//! Implémenter une méthode `infer` sur `NeuralNetwork` qui :
//! - Prend un `f64` en entrée (température unique)
//! - Retourne un `f64` (prédiction)
//! - Utilise la méthode `forward` définie précédemment
//!
//! Cette méthode est une interface simplifiée qui :
//! 1. Convertit l'entrée scalaire en vecteur `[input]`
//! 2. Appelle `forward`
//! 3. Extrait le premier (et seul) élément de la dernière activation
//!
//! ### 4.3 Test du forward pass
//!
//! Avant d'implémenter la rétropropagation, tester que le forward pass fonctionne :
//!
//! ```rust
//! // Dans une fonction de test ou main
//! let network = NeuralNetwork::new(1, vec![2], 1);
//! let input = 0.5; // Température normalisée
//! let output = network.infer(input);
//! println!("Entrée: {}, Sortie: {}", input, output);
//! ```
//!
//! **Résultat attendu** : Une valeur aléatoire (le réseau n'est pas entraîné), mais le code ne doit pas paniquer.
//!
//! ## Étape 5 : Rétropropagation - La méthode `backward`
//!
//! ### Concept général
//!
//! La rétropropagation est l'algorithme qui permet au réseau d'apprendre de ses erreurs. Après avoir fait une prédiction, on mesure à quel point on s'est trompé, puis on ajuste tous les poids du réseau pour faire mieux la prochaine fois.
//!
//! Imaginez que vous apprenez à lancer des fléchettes :
//! 1. Vous lancez (forward pass)
//! 2. Vous voyez où vous avez touché par rapport au centre (erreur)
//! 3. Vous ajustez votre position et votre geste (backward pass)
//! 4. Vous relancez avec ces ajustements
//!
//! ### Bagage théorique nécessaire
//!
//! **MSE (Mean Squared Error)** : L'erreur quadratique moyenne est notre façon de mesurer à quel point une prédiction est fausse. On calcule `(prédiction - cible)²` puis on divise par 2 pour simplifier les calculs. Le carré garantit que l'erreur est toujours positive et pénalise plus les grosses erreurs.
//!
//! **Gradient** : Le gradient est la "pente" de l'erreur. Il nous dit dans quelle direction et de combien ajuster chaque poids. Si le gradient est positif, on diminue le poids ; s'il est négatif, on l'augmente.
//!
//! **Learning rate** : C'est la taille du pas d'ajustement. Trop grand (0.5), on risque de "sauter" par-dessus la bonne solution. Trop petit (0.001), on apprend trop lentement. Une valeur de 0.1 est généralement un bon compromis.
//!
//! **Propagation de l'erreur entre couches** : Quand un neurone de la couche 2 a une erreur, cette erreur doit être redistribuée aux neurones de la couche 1 qui y ont contribué. La contribution de chaque neurone est proportionnelle à son poids de connexion. Si le poids est 2.0, il a contribué deux fois plus qu'un poids de 1.0.
//!
//! ### Description de la méthode `backward`
//!
//! Implémenter une méthode `backward` sur `NeuralNetwork` qui :
//! - Prend en paramètres :
//!   - `target: f64` : la valeur qu'on aurait dû prédire
//!   - `activations: &[Vec<f64>]` : toutes les valeurs calculées pendant le forward
//!   - `learning_rate: f64` : la force des ajustements (typiquement 0.1)
//! - Modifie directement les poids et biais du réseau (`&mut self`)
//!
//! ### Algorithme détaillé
//!
//! 1. **Calculer l'erreur de départ** :
//!    ```
//!    erreur = prédiction - target
//!    ```
//!    La prédiction est `activations.last().unwrap()[0]`
//!
//! 2. **Parcourir chaque couche en sens inverse** (de la sortie vers l'entrée) :
//!    
//!    Pour chaque couche à l'index `layer_idx` :
//!    - Récupérer les activations d'entrée : `activations[layer_idx]`
//!    - Récupérer les activations de sortie : `activations[layer_idx + 1]`
//!    - Créer un vecteur `next_error` de la taille de l'entrée, initialisé à 0
//!    
//!    Pour chaque neurone `i` de cette couche :
//!    
//!    a. **Calculer le "delta"** (l'ajustement nécessaire) :
//!       ```
//!       delta = erreur × layer.activation.derivative(output_activation[i])
//!       ```
//!       La méthode `derivative` est déjà implémentée sur `ActivationFunction`
//!    
//!    b. **Ajuster le biais** :
//!       ```
//!       biais[i] = biais[i] - (learning_rate × delta)
//!       ```
//!    
//!    c. **Pour chaque poids `j` du neurone `i`** :
//!       - **Propager l'erreur** : La contribution de ce poids à l'erreur est `poids[i][j] × delta`.
//!         L'ajouter à `next_error[j]` :
//!         ```
//!         next_error[j] += poids[i][j] × delta
//!         ```
//!       - **Ajuster le poids** :
//!         ```
//!         poids[i][j] = poids[i][j] - (learning_rate × delta × input_activation[j])
//!         ```
//!
//! 3. **Préparer l'erreur pour la prochaine itération** :
//!    - Pour un réseau multi-couches : `error = next_error[0]` (on propage seulement le premier élément car on n'a qu'une entrée)
//!    - Pour la première couche (layer_idx == 0) : on peut ignorer `next_error` car il n'y a pas de couche précédente
//!
//! <details>
//! <summary>💡 Indice : Structure complète</summary>
//!
//! ```rust
//! let mut error = activations.last().unwrap()[0] - target;
//!
//! for (layer_idx, layer) in self.layers.iter_mut().enumerate().rev() {
//!     let input_activation = &activations[layer_idx];
//!     let output_activation = &activations[layer_idx + 1];
//!     let mut next_error = vec![0.0; input_activation.len()];
//!     
//!     // Parcourir weights et biases ensemble avec zip et enumerate
//!     for (i, (weight_row, bias)) in layer.weights.iter_mut()
//!                                          .zip(&mut layer.biases)
//!                                          .enumerate() {
//!         // Calculer delta en utilisant derivative
//!         // Mettre à jour le biais
//!         // Parcourir chaque poids dans weight_row
//!         //   - Ajouter la contribution à next_error
//!         //   - Mettre à jour le poids
//!     }
//!     
//!     // Préparer error pour la prochaine couche
//!     error = if layer_idx == 0 { 0.0 } else { next_error[0] };
//! }
//! ```
//! </details>
//!
//! ### Exemple concret
//!
//! Pour mieux comprendre, prenons un exemple avec des valeurs :
//! - Erreur de sortie : 7.0
//! - Poids de la couche de sortie vers neurone 1 : 2.0
//! - Poids de la couche de sortie vers neurone 2 : 1.5
//!
//! L'erreur est redistribuée :
//! - Neurone 1 reçoit : 7.0 × 2.0 = 14.0 d'erreur
//! - Neurone 2 reçoit : 7.0 × 1.5 = 10.5 d'erreur
//!
//! Chaque neurone ajuste ensuite ses propres poids en fonction de cette erreur reçue.
//!
//! ## Étape 6 : Entraînement - La méthode `train`
//!
//! ### Concept général
//!
//! L'entraînement est le processus répétitif qui fait apprendre le réseau. Une **époque** est un passage complet sur tous les exemples d'entraînement. On fait généralement des centaines ou milliers d'époques pour que le réseau apprenne bien.
//!
//! À chaque époque :
//! 1. On présente chaque exemple au réseau
//! 2. On mesure l'erreur
//! 3. On ajuste les poids
//! 4. On recommence avec l'exemple suivant
//!
//! L'erreur moyenne devrait diminuer au fil des époques, signe que le réseau apprend.
//!
//! ### Description de la méthode `train`
//!
//! Implémenter une méthode publique `train` sur `NeuralNetwork` qui :
//! - Prend en paramètres :
//!   - `inputs: &[f64]` : toutes les températures Celsius normalisées d'entraînement
//!   - `targets: &[f64]` : les températures Fahrenheit normalisées correspondantes
//!   - `epochs: usize` : combien de fois répéter l'entraînement complet
//!   - `learning_rate: f64` : la vitesse d'apprentissage
//! - Retourne `Vec<f64>` : l'erreur moyenne de chaque époque (pour voir la progression)
//!
//! ### Algorithme détaillé
//!
//! 1. **Créer un vecteur vide pour stocker les erreurs**
//!
//! 2. **Pour chaque époque** (0 jusqu'à `epochs`) :
//!    
//!    a. Initialiser une variable `total_error` à 0.0
//!    
//!    b. **Pour chaque exemple d'entraînement** (parcourir `inputs` et `targets` ensemble) :
//!       - Faire la prédiction : appeler `forward` avec l'entrée
//!       - Calculer l'erreur MSE : `0.5 × (prédiction - cible)²`
//!       - Ajouter cette erreur à `total_error`
//!       - Ajuster les poids : appeler `backward` avec la cible et les activations
//!    
//!    c. Calculer l'erreur moyenne : `total_error / nombre_d_exemples`
//!    
//!    d. Stocker cette erreur moyenne dans le vecteur
//!    
//!    e. Afficher la progression toutes les 100 époques :
//!       ```
//!       Époque 0: Erreur moyenne = 0.245613
//!       Époque 100: Erreur moyenne = 0.004521
//!       ```
//!
//! 3. **Retourner le vecteur contenant toutes les erreurs**
//!
//! <details>
//! <summary>💡 Indice : Parcourir deux slices ensemble</summary>
//!
//! Pour parcourir `inputs` et `targets` simultanément :
//! ```rust
//! for (input, target) in inputs.iter().zip(targets) {
//!     // *input et *target sont les valeurs
//! }
//! ```
//! </details>
//!
//! ## Étape 7 : Programme principal - Le `main`
//!
//! ### Concept général
//!
//! Le programme principal assemble toutes les pièces :
//! 1. Prépare les données
//! 2. Crée le réseau
//! 3. L'entraîne
//! 4. Teste les résultats
//!
//! ### Structure détaillée du `main`
//!
//! 1. **Créer les normaliseurs** :
//!    ```rust
//!    let input_normalizer = Normalizer::new(-40.0, 40.0);
//!    let output_normalizer = Normalizer::new(-40.0, 104.0);
//!    ```
//!    Note : 104°F = 40°C × 9/5 + 32
//!
//! 2. **Générer les données d'entraînement** :
//!    - Créer des températures Celsius : -40, -35, -30, ..., 35, 40
//!    - Pour chaque température :
//!      - Calculer la température Fahrenheit correspondante
//!      - Normaliser les deux valeurs
//!    - Séparer en deux vecteurs avec `.unzip()`
//!
//! 3. **Créer et entraîner le réseau** :
//!    ```rust
//!    let mut network = NeuralNetwork::new(1, vec![2], 1);
//!    let errors = network.train(&normalized_inputs, &normalized_outputs, 1000, 0.1);
//!    ```
//!
//! 4. **Tester le réseau** :
//!    - Créer un tableau de températures de test (incluant des valeurs non vues)
//!    - Pour chaque température de test :
//!      - Normaliser avec `input_normalizer`
//!      - Prédire avec `network.infer()`
//!      - Dénormaliser avec `output_normalizer`
//!      - Calculer l'erreur absolue
//!      - Afficher dans un tableau formaté
//!
//! <details>
//! <summary>💡 Indice : Génération élégante des données</summary>
//!
//! Utiliser un itérateur avec `.step_by()` et `.map()` :
//! ```rust
//! (-40..=40).step_by(5).map(|c| {
//!     let celsius = c as f64;
//!     // Calculer fahrenheit
//!     // Normaliser les deux
//!     // Retourner un tuple
//! })
//! ```
//! </details>
//!
//! <details>
//! <summary>💡 Indice : Test sur plusieurs valeurs</summary>
//!
//! ```rust
//! let test_temperatures = [-50.0, -25.0, 0.0, 15.0, 37.0, 100.0];
//! for temp in test_temperatures {
//!     // Normaliser, prédire, dénormaliser
//!     // Comparer avec la vraie valeur
//! }
//! ```
//! </details>
//!
//! ## Résultats attendus
//!
//! Si tout est correctement implémenté :
//! - L'erreur d'entraînement devrait passer de ~0.5 à moins de 0.001
//! - Les prédictions dans la plage [-40, 40] devraient avoir moins de 1°F d'erreur
//! - Les prédictions légèrement hors plage [-50, 50] devraient rester cohérentes
//! - Les prédictions très hors plage (100°C) peuvent être imprécises (extrapolation difficile)

pub mod p_neuralnetwork;
