#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;

mod utils;

use nifti::{NiftiObject, InMemNiftiObject, NiftiVolume};
use nifti::volume::ndarray::IntoNdArray;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};
use ndarray::*;
use ndarray_stats::*;
use std::fmt;
use std::ops::Add;
use std::io::Cursor;
use std::fs::File;
use std::env;
use std::iter;
use std::f64;
use std::path::{Path, PathBuf};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Debugging in console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Testing canvas drawing with Julia set
#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    real: f64,
    imaginary: f64,
) -> Result<(), JsValue> {
    let c = Complex { real, imaginary };
    let mut data = get_julia_set(width, height, c);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_julia_set(width: u32, height: u32, c: Complex) -> Vec<u8> {
    let mut data = Vec::new();

    let param_i = 1.5;
    let param_r = 1.5;
    let scale: f64 = 0.05_f64;

    for x in 0..width {
        for y in 0..height {
            let z = Complex {
                real: y as f64 * scale - param_r,
                imaginary: x as f64 * scale - param_i,
            };
            let iter_index = get_iter_index(z, c);
            data.push((iter_index / 3) as u8);
            data.push((iter_index / 3) as u8);
            data.push((iter_index / 3) as u8);
            data.push(u8::MAX);
        }
    }

    data
}

fn get_iter_index(z: Complex, c: Complex) -> u32 {
    let mut iter_index: u32 = 0;
    let mut z = z;
    while iter_index < 900 {
        if z.norm() > 2.0 {
            break;
        }
        z = z.square() + c;
        iter_index += 1;
    }
    iter_index
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}


#[wasm_bindgen]
pub struct Lightbox {
    width: u32,
    height: u32,
    img: Vec<u8>,
}

#[wasm_bindgen]
impl Lightbox {
    pub fn new(raw_data: &[u8]) -> Lightbox {
        log(&format!("Rust raw input"));
        for byte in raw_data.iter().take(10) {
            log(&format!("{}", byte));
        }

        // TODO: figure out how to use InMemNiftiObject correctly
        // because the data gets destroyed at this point
        let obj = InMemNiftiObject::from_reader(raw_data).unwrap();
        // use obj
        let header = obj.header();
        let img = obj.into_volume().into_ndarray::<u8>().unwrap();

        log(&format!("Rust data"));
        for elem in img.iter().take(10) {
            log(&format!("{}", elem));
        }

        let max = img.max().unwrap();

        // get a slice to see what's up
        let slice = img.slice(s![.., .., 150])
            .to_slice()
            .iter()
            // JS needs 4 channels, RGBA -> repeat each element of vector
            .flat_map(|&x| {
                let scaled_x = x[0] / max * 255_u8 / 3_u8;
                iter::repeat(scaled_x).take(4)
            })
            // scale up alpha to max
            .enumerate()
            .map(|(i, x)| {
                if i > 0 && (i+1) % 4 == 0 {
                    255_u8 
                } else {
                    x
                }
            })
            .collect::<Vec<u8>>();

        log(&format!("u8 image data"));
        for elem in slice.iter().take(10) {
            log(&format!("{}", elem));
        }

        // cast to u8
        let slice = slice.iter().map(|x| *x as u8).collect::<Vec<u8>>();

        log(&format!("u8 image data"));
        for elem in slice.iter().take(10) {
            log(&format!("{}", elem));
        }
        

        Lightbox {
            width: img.shape()[0] as u32,
            height: img.shape()[1] as u32,
            img: slice,
        }
    }

    pub fn render_to_canvas(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue>{
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.img.clone()), 
            self.width, 
            self.height,
        )?;


        ctx.put_image_data(&data, 0.0, 0.0)
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
}
