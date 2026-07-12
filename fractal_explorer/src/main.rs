use fractal_explorer::{render, write_ppm, Complex, Fractal, Julia, Mandelbrot};

fn main() -> std::io::Result<()> {
    let width = 1200;
    let height = 800;
    let max_iters = 500;

    let gallery: Vec<Box<dyn Fractal + Sync>> = vec![
        Box::new(Mandelbrot),
        Box::new(Julia { c: Complex::new(-0.8, 0.156)}),
    ];

    for fractal in &gallery {
        let pixels = render(fractal.as_ref(), width, height, max_iters);
        write_ppm(&format!("{}.ppm", fractal.name()), &pixels, width, height)?;
    }
    Ok(())
}
