use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
    width: f64,
    height: f64,
    xs: Vec<f64>,
    ys: Vec<f64>,
    vxs: Vec<f64>,
    vys: Vec<f64>,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64, count: usize) -> Simulation {
        let mut xs = Vec::with_capacity(count);
        let mut ys = Vec::with_capacity(count);
        let mut vxs = Vec::with_capacity(count);
        let mut vys = Vec::with_capacity(count);

        let mut seed: u64 = 88172645463325252;
        for _ in 0..count  {
            xs.push(random(&mut seed) * width);
            ys.push(random(&mut seed) * height);
            vxs.push((random(&mut seed) - 0.5) * 4.0);
            vys.push((random(&mut seed) - 0.5) * 4.0);
        }

        Simulation { width, height, xs, ys, vxs, vys }
    }

    pub fn tick(&mut self) {
        for i in 0..self.xs.len()  {
            self.xs[i] += self.vxs[i];
            self.ys[i] += self.vys[i];

            if self.xs[i] < 0.0 || self.xs[i] > self.width {
                self.vxs[i] = -self.vxs[i];
                self.xs[i] = self.xs[i].clamp(0.0, self.width);
            }
            if self.ys[i] < 0.0 || self.ys[i] > self.height {
                self.vys[i] = -self.vys[i];
                self.ys[i] = self.ys[i].clamp(0.0, self.height);
            }
        }
    }

    pub fn positions(&self) -> Vec<f64> {
        let mut out = Vec::with_capacity(self.xs.len() * 2);
        for i in 0..self.xs.len()  {
            out.push(self.xs[i]);
            out.push(self.ys[i]);
        }
        out
    }
}

fn random(state: &mut u64) -> f64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 11) as f64 / (1u64 << 53) as f64 
}
