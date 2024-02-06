mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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
}

#[wasm_bindgen]
impl Grid {
    pub fn new() -> Grid {
        let width = 512;
        let height = 512;

        let mut pixels: Vec<Rgba> = (0..width * height)
            .map(|i| Rgba {
                r: 0,
                g: 0,
                b: 0,
                a: 255,
            })
            .collect();

        Grid {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let idx = self.get_index(i, j);

                self.pixels[idx].r = (self.pixels[idx].r + 3) % 0xff;
                self.pixels[idx].g = (self.pixels[idx].g + 2) % 0xff;
                self.pixels[idx].b = (self.pixels[idx].g + 1) % 0xff;
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
