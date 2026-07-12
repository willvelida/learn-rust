# Before you go...

* **A Complex type with operator overloading.** We built a Complex struct and implemented Add and Mul, so plus and star worked on it and z = z*z + c read like normal maths.
* **A Fractal trait, implemented twice.** Mandelbrot and Julia each wrote the same escape_time method, returning an Option for how fast a point flies off to infinity.
* **From pixels to a saved image.** We mapped every pixel into the complex plane, ran the escape-time loop, coloured the result, and wrote it all to a PPM file with an iterator.
* **Trait objects behind a thin main.** Both fractals went into one Vec of Box<dyn Fractal> in main, while all the real logic lived in the library crate, src/lib.rs.
* **Parallel and fearless.** Swapping the iterator for Rayon's into_par_iter ran the work across every core, and a Sync bound let the compiler prove there were no data races.