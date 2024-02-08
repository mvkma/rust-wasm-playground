mod utils;

use rand::prelude::*;
use std::f64::consts::PI;

use wasm_bindgen::prelude::*;

pub struct Vec2D {
    x: f64,
    y: f64,
}

pub struct FlowField {
    particles: Vec<Vec2D>,
    lifetimes: Vec<f64>,
    nparticles: u32,
    velocities: Vec<Vec<Vec2D>>,
    nsamples: u32,
    rng: ThreadRng,
}

pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub struct Grid {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
    flowfield: FlowField,
    nticks: u32,
    lifetime: u32,
}

impl Rgba {
    pub fn from_u32(n: u32) -> Rgba {
        Rgba {
            r: ((n & 0xff000000) >> 24) as u8,
            g: ((n & 0x00ff0000) >> 16) as u8,
            b: ((n & 0x0000ff00) >> 8) as u8,
            a: (n & 0x000000ff) as u8,
        }
    }
}

impl Vec2D {
    pub fn from_angle(alpha: f64) -> Vec2D {
        Vec2D {
            x: alpha.cos(),
            y: alpha.sin(),
        }
    }
}

impl FlowField {
    pub fn new(nparticles: u32, nsamples: u32) -> FlowField {
        let mut rng = rand::thread_rng();

        let particles: Vec<Vec2D> = (0..nparticles)
            .map(|_| Vec2D {
                x: rng.gen::<f64>(),
                y: rng.gen::<f64>(),
            })
            .collect();

        let lifetimes: Vec<f64> = (0..nparticles).map(|_| rng.gen::<f64>() * 100.0).collect();

        // let angle_func = |x: f64, y: f64| (x.powf(2.0) + y.powf(2.0)) * PI * 2.0;
        // let angle_func = |x: f64, y: f64| (x.powf(2.0) + y.powf(2.0) - x * y) * PI * 2.0;
        // let angle_func = |x: f64, y: f64| (x / y) * PI * 2.0;
        // let angle_func = |x: f64, y: f64| (x * y).ln() * PI * 2.0;
        let angle_func = |x: f64, y: f64| (x.powf(2.0) + y.powf(2.0)).ln() * PI * 2.0;

        let velocities = (0..nsamples)
            .map(|x| {
                (0..nsamples)
                    .map(|y| {
                        Vec2D::from_angle(angle_func(
                            f64::from(x) / f64::from(nsamples),
                            f64::from(y) / f64::from(nsamples),
                        ))
                    })
                    .collect()
            })
            .collect();

        FlowField {
            particles: particles,
            lifetimes: lifetimes,
            nparticles: nparticles,
            velocities: velocities,
            nsamples: nsamples,
            rng: rng,
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.particles.len() {
            self.lifetimes[i] -= 1.0;

            if self.lifetimes[i] <= 0.0 {
                self.particles[i].x = self.rng.gen::<f64>();
                self.particles[i].y = self.rng.gen::<f64>();
                self.lifetimes[i] = self.rng.gen::<f64>() * 100.0;
                continue;
            }

            let px = self.particles[i].x;
            let py = self.particles[i].y;

            let x = ((px * f64::from(self.nsamples)).floor() as usize) % self.velocities.len();
            let y = ((py * f64::from(self.nsamples)).floor() as usize) % self.velocities.len();

            self.particles[i].x = (px + 0.005 * self.velocities[x][y].x).clamp(0.0, 1.0);
            self.particles[i].y = (py + 0.005 * self.velocities[x][y].y).clamp(0.0, 1.0);
        }
    }
}

#[wasm_bindgen]
impl Grid {
    pub fn new() -> Grid {
        let width = 800;
        let height = 800;

        let pixels: Vec<Rgba> = (0..width * height)
            .map(|_| Rgba::from_u32(0x000000ff))
            .collect();

        let field = FlowField::new(200, 400);

        Grid {
            width: width,
            height: height,
            pixels: pixels,
            flowfield: field,
            nticks: 0,
            lifetime: 1800,
        }
    }

    pub fn tick(&mut self) {
        self.nticks += 1;

        if self.nticks % self.lifetime == 0 {
            for i in 0..self.height {
                for j in 0..self.width {
                    let idx = self.get_index(i, j);

                    self.pixels[idx].r = 0x00;
                    self.pixels[idx].g = 0x00;
                    self.pixels[idx].b = 0x00;
                }
            }
        }

        self.flowfield.tick();

        for i in 0..self.flowfield.particles.len() {
            let p = &self.flowfield.particles[i];
            let xc = (p.x * f64::from(self.width)).floor() as u32;
            let yc = (p.y * f64::from(self.height)).floor() as u32;

            self.circle(xc, yc, 1);
        }
    }

    pub fn circle(&mut self, x: u32, y: u32, r: u32) {
        for xx in (x - r)..(x + r + 1) {
            for yy in (y - r)..(y + r + 1) {
                if (xx - x).pow(2) + (yy - y).pow(2) >= r.pow(2) {
                    continue;
                }

                if xx >= self.width || yy >= self.height {
                    continue;
                }

                let idx = self.get_index(xx, yy);
                self.pixels[idx].r = 0xcc;
                self.pixels[idx].g = 0xcc;
                self.pixels[idx].b = 0xcc;
            }
        }
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn pixels(&self) -> *const Rgba {
        self.pixels.as_ptr()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &str) {
    alert(&format!("Hello, from {msg}!"));
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
