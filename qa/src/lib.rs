#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;

mod utils;

use nifti::{NiftiObject, InMemNiftiObject, NiftiVolume};
use wasm_bindgen::prelude::*;
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

        let obj = InMemNiftiObject::from_reader(raw_data).unwrap();
        // use obj
        let header = obj.header();
        let volume = obj.volume();
        let dims = volume.dim();

        log(&format!("Rust byte vec"));
        for byte in volume.raw_data().iter().take(10) {
            log(&format!("{}", byte));
        }

        let img = volume.raw_data();
        // get a 100x100 patch to see what's up
        let width = 100;
        let height = 100;
        let img = img[600..10600]
            .to_vec()
            // JS needs 4 channels, RGBA -> repeat each element of vector
            .iter()
            .flat_map(|&x| {
                let scaled_x = x / 4;
                iter::repeat(scaled_x).take(4)
            })
            // scale up alpha to max
            .enumerate()
            .map(|(i, x)| {
                if i > 0 && (i+1) % 4 == 0 {
                    u8::MAX  
                } else {
                    x
                }
            })
            .collect::<Vec<u8>>();
        

        Lightbox {
            width,
            height,
            img,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn img(&self) -> Vec<u8> {
        self.img.clone()
    }
}

impl fmt::Display for Lightbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a lightbox\n")?;
        Ok(())
    }
}
