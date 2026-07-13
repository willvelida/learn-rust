# Before you go...

* **The simulation is plain Rust.** The particle physics is ordinary Rust code, the same kind you've written all series.
* **Wasm-bindgen bridges to JavaScript.** Marking the types we expose generates the glue that lets JavaScript call into our Rust and pass values back.
* **Wasm-pack builds the WebAssembly.** One command compiles the crate to WebAssembly and packages it with the JavaScript glue, ready to import like any web module.
* **Rust computes, JavaScript draws.** Each frame, Rust advances the physics and JavaScript paints the positions onto a canvas.