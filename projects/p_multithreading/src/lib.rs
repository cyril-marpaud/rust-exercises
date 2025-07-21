//! # Exercice : Calcul parallèle de la fractale de Mandelbrot
//!
//! Dans cet exercice, nous allons implémenter le calcul et le rendu de l'[ensemble de Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set), d'abord de manière séquentielle, puis en utilisant différentes approches de parallélisation pour améliorer les performances. Nous traçerons également un graphe des performances pour comparer ces approches.
//!
//! ## Introduction : la fractale de Mandelbrot
//!
//! La fractale de Mandelbrot est un ensemble de [nombres complexes](https://en.wikipedia.org/wiki/Complex_number) défini par une relation de récurrence.
//!
//! Pour chaque point $c$ du plan complexe, on calcule la suite : $z_{n+1} = z_n^2 + c$ avec $z_0 = 0$. Si cette suite reste bornée ($|z_n| \leq 2$, c'est à dire si [son module](https://en.wikipedia.org/wiki/Absolute_value#Complex_numbers) reste inférieur à deux), alors $c$ appartient à l'ensemble de Mandelbrot.
//!
//! Pour visualiser la fractale, nous allons :
//! - Définir une zone rectangulaire du plan complexe à explorer (délimitée par deux nombres complexes)
//! - Créer une image de taille donnée en pixels
//! - Pour chaque pixel, calculer le nombre d'itérations avant que la suite ne diverge
//! - Utiliser ce nombre pour déterminer la couleur du pixel
//!
//! ## Partie 1 : Mise en place
//!
//! ### 1. Récupération des arguments du programme
//!
//! Le programme attend 3 arguments :
//! - La taille de l'image en pixels au format `WIDTHxHEIGHT` (ex: `1920x1080`)
//! - Le coin supérieur gauche du plan complexe au format `RE,IM` (ex: `-2.5,1.0`)
//! - Le coin inférieur droit du plan complexe au format `RE,IM` (ex: `1.0,-1.0`)
//!
//! Exemple :
//!
//! ```bash
//! cargo run --release -- 1920x1080 -2.5,1.0 1.0,-1.0
//! ```
//!
//! Dans le fichier `main.rs`, utiliser [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) pour récupérer les arguments du programme. Cette fonction renvoie un itérateur sur les arguments passés en ligne de commande dont le premier est le nom du programme invoqué.
//!
//! ### 2. Implémentation de `parse_pair`
//!
//! Cette fonction doit parser une chaîne de caractères contenant deux valeurs numériques séparées par un caractère donné. Elle retourne `Some<(T, T)>` en cas de succès, `None` sinon. Pour cela, utiliser :
//! - Le trait [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) qui représente les types pouvant être construits depuis une chaine de caractères
//! - La fonction [`parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse).
//!
//! <details>
//! <summary>💡 Indice : Prototype de la fonction</summary>
//!
//! ```rust
//! fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)>
//! ```
//!
//! </details>
//!
//! ### 3. Implémentation de `parse_cpx`
//!
//! Cette fonction utilise `parse_pair` pour parser un nombre complexe au format `RE,IM` et retourne `Option<Complex<f64>>`.
//! Le type [`Complex`](https://docs.rs/num/latest/num/struct.Complex.html) provient du crate [`num`](https://docs.rs/num/) qui a déjà été ajouté aux dépendances du projet.
//!
//! <details>
//! <summary>💡 Indice : Utilisation de parse_pair</summary>
//!
//! Appeler `parse_pair::<f64>(s, ',')` et transformer le résultat en `Complex`.
//! </details>
//!
//! ### 4. Structure Image et constructeur
//!
//! La structure `Image` se constitue de deux champs :
//! - `buf`: un `Vec<u8>` représentant les pixels de l'image ligne par ligne
//! - `cfg`: une structure `ImageCfg` contenant les paramètres de l'image
//!
//! Implémenter le constructeur `Image::new` prenant un `ImageCfg` en paramètre et initialisant le buffer avec la bonne taille (width × height pixels).
//!
//! ### 5. Utilisation dans main
//!
//! Dans la fonction `main`, utiliser ces fonctions de parsing pour construire une `Image` vide à partir des arguments du programme.
//!
//! ### 6. Implémentation de `cpx_to_escape`
//!
//! Cette fonction prend un nombre complexe en paramètre et renvoie un nombre entier entre 0 et [`u8::MAX`](https://doc.rust-lang.org/std/primitive.u8.html#associatedconstant.MAX) dont la valeur nous permettra d'encoder la couleur de chaque pixel en niveaux de gris sur un octet.
//!
//! Elle procède en calculant le nombre d'itérations nécessaires pour que la suite diverge. Si la suite n'a pas divergé après 256 itérations, on considère que le complexe analysé appartient bien à la fractale et on l'affiche en noir pur.
//!
//! La suite diverge quand $|z| > 2$, c'est à dire quand la [norme du nombre complexe](https://en.wikipedia.org/wiki/Norm_(mathematics)) est supérieure à 2. La norme d'un complexe peut être calculée grâce aux méthodes [`norm`](https://docs.rs/num/latest/num/struct.Complex.html#method.norm) ou [`norm_sqr`](https://docs.rs/num/latest/num/struct.Complex.html#method.norm_sqr).
//!
//! <details>
//! <summary>💡 Indice : Algorithme</summary>
//!
//! ```rust
//! // Étant donnés c (le complexe analysé) et z (un complexe initialisé à 0), itérer de 0 à u8::MAX
//! // À chaque itération, vérifier si la norme de z est supérieure à 2
//! // Si oui, retourner le nombre d'itérations
//! // Sinon, calculer `z = z * z + c` et passer à l'itération suivante
//! // Si la suite ne diverge pas après 256 itérations, renvoyer `u8::MAX`
//! ```
//! </details>
//!
//! ### 7. Implémentation de `pxl_to_cpx`
//!
//! Cette méthode convertit les coordonnées d'un pixel (row, col) en nombre complexe dans le plan défini par `ImageCfg`. Elle doit faire la correspondance entre :
//! - `(0, 0)` → `top_left`
//! - `(height-1, width-1)` → `bot_right`
//!
//! <details>
//! <summary>💡 Indice : Formules de conversion</summary>
//!
//! Pour un pixel à la position (row, col) :
//! - Largeur complexe de l'image : `cpx_w = bot_right.re - top_left.re`
//! - Hauteur complexe de l'image : `cpx_h = top_left.im - bot_right.im`
//! - La partie réelle : `re = top_left.re + cpx_w * (col / width)`
//! - La partie imaginaire : `im = top_left.im - cpx_h * (row / height)`
//! </details>
//!
//! ### 8. Implémentation de `pxl_to_escape`
//!
//! Cette méthode combine `pxl_to_cpx` et `cpx_to_escape` pour calculer le niveau de gris d'un pixel. Un point appartenant à la fractale (escape time >= 255) doit apparaître en noir (encodé par un 0) tandis qu'un point pour lequel la suite diverge doit être plus clair (encodé par un nombre entre 1 et 255).
//!
//! ### 9. Implémentation de `write`
//!
//! Cette méthode utilise le crate [`image`](https://docs.rs/image/) pour sauvegarder l'image au format PNG.
//! Vous devez utiliser :
//! - [`File::create(filename)`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) pour créer le fichier
//! - [`PngEncoder::new(file)`](https://docs.rs/image/latest/image/codecs/png/struct.PngEncoder.html) pour créer l'encodeur
//! - La méthode [`write_image`](https://docs.rs/image/latest/image/trait.ImageEncoder.html#tymethod.write_image) du trait [`ImageEncoder`](https://docs.rs/image/latest/image/trait.ImageEncoder.html)
//! - Le type de couleur [`ExtendedColorType::L8`](https://docs.rs/image/latest/image/enum.ExtendedColorType.html#variant.L8) (niveaux de gris sur 8 bits)
//!
//! ## Partie 2 : Rendu
//!
//! Maintenant que nous avons les moyens de construire une image à partir des arguments passés en ligne de commande, il est temps de passer au rendu. Nous commencerons par un rendu séquentiel, puis nous ajouterons la bibliothèque [`rayon`](https://docs.rs/rayon/) au projet pour paralléliser facilement le rendu et enfin, nous terminerons avec une implémentation manuelle utilisant des threads.
//!
//! ### 1. Mesure du temps d'exécution
//!
//! Pour mesurer les performances de nos implémentations, nous allons mesurer le temps d'exécution de chaque fonction de rendu. Pour cela, chaque fonction de rendu doit :
//! - Initialiser un [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) avec la fonction [`now`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now) pour capturer l'instant de départ
//! - Appeler la méthode [`elapsed`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed) une fois les calculs terminés pour renvoyer la durée écoulée (type [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html))
//!
//! Dans le main, on utilisera ensuite la méthode [`as_secs_f64`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs_f64) pour afficher cette durée en secondes
//!
//! ### 2. Implémentation de `render` séquentiel
//!
//! Cette méthode parcoure successivement chaque pixel de l'image et appelle `pxl_to_escape` pour en calculer la couleur.
//!
//! <details>
//! <summary>💡 Indice : Accès au buffer</summary>
//!
//! Pour un pixel à la position (row, col), l'index dans le buffer est :
//! `index = row * width + col`
//! </details>
//!
//! ### 3. Implémentation de `render` parallèle par lignes avec Rayon
//!
//! Le crate `rayon` permet de paralléliser facilement les opérations sur les collections. Parmi les outils proposés, on pourra notamment utiliser :
//!  - La méthode [`par_chunks_exact_mut`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_chunks_exact_mut) du trait [`ParallelSliceMut`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html) qui divise une slice en morceaux mutables de taille fixe.
//! - La méthode [`enumerate`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html#method.enumerate) du trait [`IndexedParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html)
//! - La méthode [`for_each`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.for_each) du trait [`ParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html)
//!
//! Implémenter `render_parallel_lines` en découpant le buffer par lignes puis en calculant la couleur de chaque pixel pour une ligne donnée.
//!
//! ### 4. Implémentation de `render` parallèle par pixels avec Rayon
//!
//! Le principe est ici le même que dans la section précédente, mais on découpe cette fois le buffer pixel par pixel en appelant `par_chunks_exact_mut(1)`.
//!
//! <details>
//! <summary>💡 Indice : Conversion index → coordonnées</summary>
//!
//! Pour un index `i` dans le buffer :
//! - `row = i / width`
//! - `col = i % width`
//! </details>
//!
//! ### 5. Implémentation de `render` parallèle manuellement avec des scoped threads
//!
//! Contrairement aux threads classiques, les scoped threads sont capables d'emprunter des références non statiques à leur contexte. La fonction [`scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) permet notamment de créer un scope à la fin duquel nous avons la garantie que l'exécution de chaque thread est terminée.
//!
//! Implémenter `render_parallel_lines_scoped` :
//! - Le nombre de threads (`nb_threads`) que la fonction va lancer lui sera passé directement en argument
//! - Utiliser [`chunks_exact_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact_mut) pour créer un itérateur de chunks (cad de lignes) mutables sur le buffer à traiter
//! - Protéger cet itérateur des accès concurrents avec un [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
//! - Utiliser la fonction `scope` pour créer un scope
//! - Dans ce scope, utiliser la fonction [`spawn`](https://doc.rust-lang.org/std/thread/struct.Scope.html#method.spawn) dans une boucle pour lancer les threads
//! - Chaque thread doit :
//!   - Verrouiller le mutex, récupérer un chunk (une ligne), et déverrouiller le mutex aussi vite que possible
//!   - Traiter la ligne obtenue en calculant la couleur de chaque pixel (comme dans la fonction `render_parallel_lines`)
//!   - Recommencer jusqu'à épuisement des chunks
//!
//! ### 6. Vérifications
//!
//! Pour chaque fonction implémentée, afficher le temps d'exécution mesuré et en vérifier la cohérence. Vérifier également que toutes les images générées soient bien identiques.
//!
//! ## Partie 3 : Visualisation et analyse des performances
//!
//! ### 1. Le crate poloto
//!
//! Poloto permet de créer des graphiques SVG, nous allons l'exploiter pour tracer la courbe des performances de la fonction `render_parallel_lines_scoped` en fonction du nombre de threads qu'elle exécute.
//!
//! Les principaux éléments à utiliser sont :
//! - `plot("nom").line(data)` pour créer une série de données
//! - `poloto::frame_build()` pour construire le graphique
//! - `.data([series1, series2, ...])` pour ajouter les séries
//! - `.build_and_label((titre, x_label, y_label))` pour les labels
//! - `.render_io_write(file)` pour écrire dans un fichier
//!
//! ### 2. Collecte des données de performance
//!
//! Dans `main`, après avoir testé les différentes méthodes de rendu :
//! - Créer des vecteurs pour stocker les durées et ratios
//! - Tester `render_parallel_lines_scoped` avec 1 à MAX_THREADS threads
//! - Pour chaque test, calculer le gain de performance (durée d'exécution séquentielle divisée par la durée en parallèle)
//!
//! ### 3. Création du graphique
//!
//! Créez trois séries de données :
//! - Les ratios en fonction du nombre de threads
//! - Les durées en fonction du nombre de threads
//! - La première bissectrice (la courbe d'équation $y = x$)
//!
//! ### 4.Analyse
//!
//! - Les temps d'exécution et ratios recueillis doivent être cohérents
//! - Le graphique SVG généré doit présenter une première partie quasi linéaire suivie d'un plateau
//! - Les images générées doivent être identiques
//!   - Bonus : utiliser le crate [`blake3`](https://docs.rs/blake3/) pour calculer et comparer les hashs des fichiers générés
//!
//! Questions à se poser :
//! - Pourquoi la parallélisation par pixels est-elle moins efficace ?
//! - À partir de combien de threads le gain devient-il marginal ?
//! - Pourquoi le ratio n'atteint-il pas la première bissectrice ?

pub mod p_multithreading;
