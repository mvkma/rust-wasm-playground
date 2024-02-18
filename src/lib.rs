mod fields;
mod utils;

use rand::prelude::*;

use wasm_bindgen::prelude::*;

use crate::fields::FIELDS;
use crate::utils::set_panic_hook;

#[derive(Debug)]
pub struct Vec2D {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct Particle {
    position: Vec2D,
    lifetime: f64,
}

#[derive(Debug)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub struct FlowField {
    particles: Vec<Particle>,
    velocities: Vec<Vec<Vec2D>>,
    nsamples: u32,
    rng: ThreadRng,
    max_particle_lifetime: f64,
}

#[wasm_bindgen]
pub struct Grid {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
    flowfield: FlowField,
    nticks: u32,
    grid_lifetime: u32,
    fields: [FlowFieldFunction; FIELDS.len()],
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct FlowFieldFunction {
    name: &'static str,
    func: fn(f64, f64) -> Vec2D,
}

#[wasm_bindgen]
impl FlowFieldFunction {
    pub fn name(&self) -> String {
        String::from(self.name)
    }
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

    pub fn random(rng: &mut ThreadRng) -> Vec2D {
        Vec2D {
            x: rng.gen::<f64>(),
            y: rng.gen::<f64>(),
        }
    }

    pub fn norm(&self, n: i32) -> f64 {
        (self.x.powi(n) + self.y.powi(n)).sqrt()
    }

    pub fn add(&mut self, other: &Vec2D) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn mul(&mut self, alpha: f64) {
        self.x *= alpha;
        self.y *= alpha;
    }

    pub fn clamp(&mut self, min: f64, max: f64) {
        self.x = self.x.clamp(min, max);
        self.y = self.y.clamp(min, max);
    }
}

impl FlowField {
    pub fn new(
        nparticles: u32,
        nsamples: u32,
        flow_field_function: &FlowFieldFunction,
        max_lifetime: f64,
    ) -> FlowField {
        let mut rng = rand::thread_rng();
        let dt = 0.005;

        let particles: Vec<Particle> = (0..nparticles)
            .map(|_| Particle {
                position: Vec2D::random(&mut rng),
                lifetime: rng.gen::<f64>() * max_lifetime,
            })
            .collect();

        let velocities: Vec<Vec<Vec2D>> = (0..nsamples)
            .map(|i| {
                (0..nsamples)
                    .map(|j| {
                        // The offset here is needed to make the picture symmetric with a low number of samples.
                        let mut v = (flow_field_function.func)(
                            (f64::from(i) + 0.5) / f64::from(nsamples),
                            (f64::from(j) + 0.5) / f64::from(nsamples),
                        );
                        v.mul(dt);
                        v
                    })
                    .collect()
            })
            .collect();

        FlowField {
            particles: particles,
            velocities: velocities,
            nsamples: nsamples,
            rng: rng,
            max_particle_lifetime: max_lifetime,
        }
    }

    pub fn tick(&mut self) {
        let n = f64::from(self.nsamples);

        for p in self.particles.iter_mut() {
            p.lifetime -= 1.0;

            // Reset particle to a new random position
            if p.lifetime <= 0.0 {
                p.position = Vec2D::random(&mut self.rng);
                p.lifetime = self.rng.gen::<f64>() * self.max_particle_lifetime;
                continue;
            }

            let i = (p.position.x * n).clamp(0.0, n - 1.0).floor() as usize;
            let j = (p.position.y * n).clamp(0.0, n - 1.0).floor() as usize;

            p.position.add(&self.velocities[i][j]);
            p.position.clamp(0.0, 1.0);
        }
    }
}

#[wasm_bindgen]
impl Grid {
    pub fn new(
        width: u32,
        height: u32,
        nparticles: u32,
        nsamples: u32,
        lifetime: u32,
        func: usize,
        max_lifetime: u32,
    ) -> Grid {
        set_panic_hook();

        let pixels: Vec<Rgba> = (0..width * height)
            .map(|_| Rgba::from_u32(0x000000ff))
            .collect();

        // let fields = FIELDS.to_vec();

        let field = FlowField::new(nparticles, nsamples, &FIELDS[func], f64::from(max_lifetime));

        Grid {
            width: width,
            height: height,
            pixels: pixels,
            flowfield: field,
            nticks: 0,
            grid_lifetime: lifetime,
            fields: FIELDS,
        }
    }

    pub fn set_flow_params(
        &mut self,
        nparticles: u32,
        nsamples: u32,
        func: usize,
        max_lifetime: u32,
    ) {
        let field = FlowField::new(
            nparticles,
            nsamples,
            &self.fields[func],
            f64::from(max_lifetime),
        );

        self.clear();

        self.flowfield = field;
    }

    pub fn set_grid_lifetime(&mut self, lifetime: u32) {
        self.grid_lifetime = lifetime;
        self.nticks = 0;
    }

    pub fn clear(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.get_index(i, j);

                self.pixels[idx].r = 0x00;
                self.pixels[idx].g = 0x00;
                self.pixels[idx].b = 0x00;
            }
        }
    }

    pub fn tick(&mut self) {
        self.nticks += 1;

        if self.nticks % self.grid_lifetime == 0 {
            self.clear();
        }

        self.flowfield.tick();

        for i in 0..self.flowfield.particles.len() {
            let p = &self.flowfield.particles[i];
            let xc = (p.position.x * f64::from(self.width)).floor() as u32;
            let yc = (p.position.y * f64::from(self.height)).floor() as u32;

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
                self.pixels[idx].r = 0xf9;
                self.pixels[idx].g = 0xa9;
                self.pixels[idx].b = 0x00;
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

    pub fn fields(&self) -> Vec<FlowFieldFunction> {
        self.fields.to_vec()
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
