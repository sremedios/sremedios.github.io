#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;

mod utils;

use image::{DynamicImage, GenericImageView, ImageFormat, io::Reader};
use wasm_bindgen::prelude::*;
use std::fmt;
use std::io::Cursor;
use std::fs::File;
use std::env;
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
    img: image::RgbaImage,
}

#[wasm_bindgen]
impl Lightbox {
    pub fn new(in_bytes: Vec<u8>) -> Lightbox {
        log("Entered constructor");
        log(&format!("length: {}", in_bytes.len()));
        log(&format!("raw bytes"));
        for byte in in_bytes.iter().take(10) {
            log(&format!("{} ", byte));
        }
        let img = image::load_from_memory_with_format(&in_bytes, ImageFormat::Png).unwrap();
        log(&format!("Image decoded"));

        log("After loading image");
        let (width, height) = img.dimensions();
        log(&format!("{}", width));
        log(&format!("{}", height));
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

    // TODO: figure out how to return some kind of image that HTML canvas
    // can draw. Maybe HTML canvas can take a bytestring?
    pub fn img(&self) -> image::RgbaImage {
        self.img.as_rgba8.buffer
    }
}

impl fmt::Display for Lightbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a lightbox\n")?;
        Ok(())
    }
}
