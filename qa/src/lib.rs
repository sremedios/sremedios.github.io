extern crate image;

mod utils;

use image::{DynamicImage, GenericImageView};
use wasm_bindgen::prelude::*;
use std::fmt;
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
    img: DynamicImage,
}

#[wasm_bindgen]
impl Lightbox {
    pub fn new(input_path: &str) -> Lightbox {
        log(input_path);
        log("Before loading image");
        let img_path = Path::new(input_path);
        log(&format!("{}", img_path.exists()));


        log("Before cwd");
        let cwd = env::current_dir();
        match cwd {
            Ok(p) => {
                log(&format!("{}", p.exists()));
                log(&format!("{}", p.display()));
            },
            Err(e) => log(&format!("Some error: {}", e)),
        }
        log("After cwd");



        let img = image::open(&img_path).unwrap();
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
}

impl fmt::Display for Lightbox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is a lightbox\n")?;
        Ok(())
    }
}
