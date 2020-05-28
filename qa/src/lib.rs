#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;

mod utils;

use nifti::{NiftiObject, InMemNiftiObject, NiftiVolume};
use nifti::volume::ndarray::IntoNdArray;
use wasm_bindgen::prelude::*;
use ndarray::*;
use ndarray_stats::*;
use std::fmt;
use std::io::Cursor;
use std::fs::File;
use std::env;
use std::iter;
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

#[wasm_bindgen]
pub struct Lightbox {
    width: usize,
    height: usize,
    img: Vec<u16>,
}

#[wasm_bindgen]
impl Lightbox {
    pub fn new(raw_data: &[u8]) -> Lightbox {
        log(&format!("Rust raw input"));
        for byte in raw_data.iter().take(10) {
            log(&format!("{}", byte));
        }

        let obj = InMemNiftiObject::from_reader(raw_data).unwrap();
        // use obj
        let header = obj.header();
        let img = obj.into_volume().into_ndarray::<u16>().unwrap();

        log(&format!("Rust data"));
        for elem in img.iter().take(10) {
            log(&format!("{}", elem));
        }

        let max = img.max().unwrap();

        // get a slice to see what's up
        let slice = img.slice(s![.., .., 150])
            .to_slice()
            // JS needs 4 channels, RGBA -> repeat each element of vector
            .iter()
            .flat_map(|&x| {
                let scaled_x = x[0] / max * 255 / 4;
                iter::repeat(scaled_x).take(4)
            })
            // scale up alpha to max
            .enumerate()
            .map(|(i, x)| {
                if i > 0 && (i+1) % 4 == 0 {
                    u16::MAX  
                } else {
                    x
                }
            })
            .collect::<Vec<u16>>();
        

        Lightbox {
            width: img.shape()[0],
            height: img.shape()[1],
            img: slice,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn img(&self) -> Vec<u16> {
        self.img.clone()
    }
}

impl fmt::Display for Lightbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a lightbox\n")?;
        Ok(())
    }
}
