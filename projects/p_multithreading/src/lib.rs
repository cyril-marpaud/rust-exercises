//! <img src="../../../assets/mandelbrot.png" alt="Mandelbrot fractal" style="display: block; margin: auto; max-width: 300px">
//!
//! # Parallel Computation of the Mandelbrot Fractal
//!
//! In this exercise, we'll implement the computation and rendering of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set), first sequentially, then using different parallelization approaches to improve performance. We'll also plot a performance graph to compare these approaches.
//!
//! ## Introduction: The Mandelbrot Fractal
//!
//! The Mandelbrot fractal is a set of [complex numbers](https://en.wikipedia.org/wiki/Complex_number) defined by a recurrence relation.
//!
//! For each point $c$ in the complex plane, we compute the sequence: $z_{n+1} = z_n^2 + c$ with $z_0 = 0$. If this sequence remains bounded ($|z_n| \leq 2$, meaning its [modulus](https://en.wikipedia.org/wiki/Absolute_value#Complex_numbers) stays below two), then $c$ belongs to the Mandelbrot set.
//!
//! To visualize the fractal, we will:
//! - Define a rectangular area of the complex plane to explore (bounded by two complex numbers)
//! - Create an image of a given size in pixels
//! - For each pixel, calculate the number of iterations before the sequence diverges
//! - Use this number to determine the pixel's color
//!
//! ## Part 1: Setup
//!
//! ### 1. Retrieving Command Line Arguments (`main.rs`)
//!
//! The program expects 3 arguments:
//! - The image size in pixels in the format `WIDTHxHEIGHT` (e.g., `1920x1080`)
//! - The top-left corner of the complex plane in the format `RE,IM` (e.g., `-2.5,1.0`)
//! - The bottom-right corner of the complex plane in the format `RE,IM` (e.g., `1.0,-1.0`)
//!
//! Example:
//!
//! ```bash
//! cargo run --release -- 1920x1080 -2.5,1.0 1.0,-1.0
//! ```
//!
//! Use [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html) to retrieve the program arguments. This function returns an iterator over the command line arguments, where the first is the name of the invoked program. Once you have the iterator, the next steps describe how to parse these arguments.
//!
//! ### 2. Implementing `parse_pair` (`mod.rs`)
//!
//! This function must parse a string containing two numeric values separated by a given character. It returns `Some<(T, T)>` on success, `None` otherwise. To achieve this, use:
//! - The [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait which represents types that can be constructed from a string
//! - The [`parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) function.
//!
//! <details>
//! <summary>💡 Hint: Function prototype</summary>
//!
//! ```ignore
//! fn parse_pair<T: FromStr>(s: &str, sep: char) -> Option<(T, T)>
//! ```
//!
//! </details>
//!
//! ### 3. Implementing `parse_cpx` (`mod.rs`)
//!
//! This function uses `parse_pair` to parse a complex number in the format `RE,IM` and returns `Option<Complex<f64>>`.
//! The [`Complex`](https://docs.rs/num/latest/num/struct.Complex.html) type comes from the [`num`](https://docs.rs/num/) crate which has already been added to the project dependencies.
//!
//! <details>
//! <summary>💡 Hint: Using parse_pair</summary>
//!
//! Call `parse_pair::<f64>(s, ',')` and transform the result into a `Complex`.
//! </details>
//!
//! ### 4. Image Structure and Constructor (`image.rs`)
//!
//! The `Image` structure consists of two fields:
//! - `buf`: a `Vec<u8>` representing the image pixels row by row
//! - `cfg`: an `ImageCfg` structure containing the image parameters
//!
//! Implement the `Image::new` constructor taking an `ImageCfg` parameter and initializing the buffer with the correct size (width × height pixels).
//!
//! ### 5. Building an actual image (`main.rs`)
//!
//! In the `main` function, use these parsing functions to construct an empty `Image` from the program arguments.
//!
//! ### 6. Implementing `cpx_to_escape` (`mod.rs`)
//!
//! This function takes a complex number as a parameter and returns an integer between 0 and [`u8::MAX`](https://doc.rust-lang.org/std/primitive.u8.html#associatedconstant.MAX) representing how quickly the Mandelbrot sequence diverges for that point. This value will be used to encode each pixel's color in grayscale on one byte.
//!
//! The algorithm works by iterating the Mandelbrot equation $z_{n+1} = z_n^2 + c$ starting from $z_0 = 0$. For each complex number $c$, we count how many iterations it takes before the sequence "escapes" (diverges to infinity). Points that never escape are part of the Mandelbrot set.
//!
//! We can detect divergence by checking if $|z|^2 > 4$ - once the squared magnitude exceeds 4, the sequence will inevitably grow without bound. The [`norm_sqr`](https://docs.rs/num/latest/num/struct.Complex.html#method.norm_sqr) method efficiently calculates the squared magnitude without computing a square root.
//!
//! If the sequence hasn't escaped after 256 iterations, we assume the point belongs to the Mandelbrot set and return `u8::MAX`.
//!
//! <details>
//! <summary>💡 Hint: Iteration loop</summary>
//!
//! You'll need:
//! - A mutable `z` starting at 0
//! - A loop from 0 to `u8::MAX`
//! - An escape condition checking if `z.norm_sqr() > 4.0`
//! - The recurrence formula: `z = z * z + c`
//! </details>
//!
//! ### 7. Implementing `pxl_to_cpx` (`image_cfg.rs`)
//!
//! This method converts pixel coordinates (row, col) to a complex number in the plane defined by `ImageCfg`. The conversion must correctly map the discrete pixel grid to the continuous complex plane region.
//!
//! <details>
//! <summary>💡 Hint: Linear mapping</summary>
//!
//! Consider which pixels should map to which complex numbers:
//! - Top-left pixel (0, 0) → `top_left`
//! - Bottom-right pixel (height-1, width-1) → `bot_right`
//! - Everything in between needs linear interpolation
//!
//! Remember: image coordinates have Y increasing downward (as opposed to mathematical plots where Y increases upward).
//! </details>
//!
//! ### 8. Implementing `pxl_to_escape` (`image_cfg.rs`)
//!
//! This method combines `pxl_to_cpx` and `cpx_to_escape` to calculate a pixel's grayscale level. A point belonging to the fractal (escape time >= 255) should appear black (encoded as 0) while a point where the sequence diverges should be lighter (encoded as a number between 1 and 255).
//!
//! ### 9. Implementing `write` (`image.rs`)
//!
//! This method uses the [`image`](https://docs.rs/image/) crate to save the image in PNG format.
//!
//! First, add this crate to the project, then use:
//! - [`File::create(filename)`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create) to create the file
//! - [`PngEncoder::new(file)`](https://docs.rs/image/latest/image/codecs/png/struct.PngEncoder.html) to create the encoder
//! - The [`write_image`](https://docs.rs/image/latest/image/trait.ImageEncoder.html#tymethod.write_image) method from the [`ImageEncoder`](https://docs.rs/image/latest/image/trait.ImageEncoder.html) trait
//! - The color type [`ExtendedColorType::L8`](https://docs.rs/image/latest/image/enum.ExtendedColorType.html#variant.L8) (8-bit grayscale)
//!
//! ## Part 2: Rendering
//!
//! Now that we have the means to build an image from command line arguments, it's time to move on to rendering. We'll start with sequential rendering, then add the [`rayon`](https://docs.rs/rayon/) library to the project to easily parallelize rendering, and finally, we'll finish with a manual implementation using threads.
//!
//! ### 1. Measuring Execution Time
//!
//! To measure the performance of our implementations, we'll measure the execution time of each render function. To do this, each render function must:
//! - Initialize an [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html) with the [`now`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.now) function to capture the start time
//! - Call the [`elapsed`](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed) method once calculations are complete to return the elapsed duration ([`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) type)
//!
//! In main, we'll then use the [`as_secs_f64`](https://doc.rust-lang.org/std/time/struct.Duration.html#method.as_secs_f64) method to display this duration in seconds
//!
//! ### 2. Implementing Sequential `render` (`image.rs`)
//!
//! This method iterates through each pixel of the image sequentially and calls `pxl_to_escape` to calculate its color.
//!
//! <details>
//! <summary>💡 Hint: Buffer access</summary>
//!
//! For a pixel at position (row, col), the index in the buffer is:
//! `index = row * width + col`
//! </details>
//!
//! ### 3. Implementing Parallel `render` by Lines with Rayon (`image.rs`)
//!
//! The `rayon` crate allows easy parallelization of operations on collections. Among the tools provided, we can use:
//!  - The [`par_chunks_exact_mut`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html#method.par_chunks_exact_mut) method from the [`ParallelSliceMut`](https://docs.rs/rayon/latest/rayon/slice/trait.ParallelSliceMut.html) trait which divides a slice into mutable chunks of fixed size.
//! - The [`enumerate`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html#method.enumerate) method from the [`IndexedParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.IndexedParallelIterator.html) trait
//! - The [`for_each`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.for_each) method from the [`ParallelIterator`](https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html) trait
//!
//! Implement `render_parallel_lines` by splitting the buffer by lines then calculating the color of each pixel for a given line.
//!
//! ### 4. Implementing Parallel `render` by Pixels with Rayon (`image.rs`)
//!
//! The principle here is the same as in the previous section, but this time we split the buffer pixel by pixel by calling `par_chunks_exact_mut(1)`.
//!
//! <details>
//! <summary>💡 Hint: Index → coordinates conversion</summary>
//!
//! For an index `i` in the buffer:
//! - `row = i / width`
//! - `col = i % width`
//! </details>
//!
//! ### 5. Implementing Parallel `render` Manually with Scoped Threads (`image.rs`)
//!
//! Unlike classic threads, scoped threads can borrow non-static references from their context. The [`scope`](https://doc.rust-lang.org/std/thread/fn.scope.html) function notably allows creating a scope at the end of which we have the guarantee that each thread's execution is complete.
//!
//! Implement `render_parallel_lines_scoped`:
//! - The number of threads (`nb_threads`) that the function will launch will be passed directly as an argument
//! - Use [`chunks_exact_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact_mut) to create an iterator of mutable chunks (i.e., lines) on the buffer to process
//! - Protect this iterator from concurrent access with a [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
//! - Use the `scope` function to create a scope
//! - In this scope, use the [`spawn`](https://doc.rust-lang.org/std/thread/struct.Scope.html#method.spawn) function in a loop to launch threads
//! - Each thread must:
//!   - Lock the mutex, retrieve a chunk (a line), and unlock the mutex as quickly as possible
//!   - Process the obtained line by calculating the color of each pixel (as in the `render_parallel_lines` function)
//!   - Repeat until chunks are exhausted
//!
//! ### 6. Verification
//!
//! For each implemented function, display the measured execution time and verify its consistency. Also verify that all generated images are identical.
//!
//! ## Part 3: Performance Visualization and Analysis
//!
//! ### 1. The poloto Crate
//!
//! Poloto allows creating SVG graphs. We'll use it to plot the performance curve of the `render_parallel_lines_scoped` function as a function of the number of threads it executes.
//!
//! The main elements to use are:
//! - `plot("name").line(data)` to create a data series
//! - `poloto::frame_build()` to build the graph
//! - `.data([series1, series2, ...])` to add the series
//! - `.build_and_label((title, x_label, y_label))` for labels
//! - `.render_io_write(file)` to write to a file
//!
//! ### 2. Collecting Performance Data
//!
//! In `main`, after testing the different render methods:
//! - Create vectors to store durations and ratios
//! - Test `render_parallel_lines_scoped` with 1 to MAX_THREADS threads
//! - For each test, calculate the performance gain (sequential execution duration divided by parallel duration)
//!
//! ### 3. Creating the Graph
//!
//! Create three data series:
//! - Ratios as a function of the number of threads
//! - Durations as a function of the number of threads
//! - The first bisector (the curve with equation $y = x$)
//!
//! ### 4. Analysis
//!
//! - The collected execution times and ratios should be consistent
//! - The generated SVG graph should show a quasi-linear first part followed by a plateau
//! - The generated images should be identical
//!   - Bonus: use the [`blake3`](https://docs.rs/blake3/) crate to calculate and compare the hashes of the generated files
//!
//! Questions to consider:
//! - Why is pixel-by-pixel parallelization less efficient?
//! - At how many threads does the gain become marginal?
//! - Why doesn't the ratio reach the first bisector?

// pub mod p_multithreading;
