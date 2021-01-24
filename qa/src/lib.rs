#![allow(dead_code)]
#![allow(unused_imports)]
extern crate image;

mod utils;

use base64::{decode, encode};
use image::DynamicImage::ImageRgba8;
use image::{GenericImage, GenericImageView};
use ndarray::*;
use ndarray_stats::*;
use nifti::volume::ndarray::IntoNdArray;
use nifti::{InMemNiftiObject, NiftiObject, NiftiVolume};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Lightbox {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Lightbox {
    pub fn new(raw_pixels: Vec<u8>, width: u32, height: u32) -> Lightbox {
        Lightbox {
            raw_pixels,
            width,
            height,
        }
    }

    /*
        log(&format!("Rust raw input"));
        for byte in raw_data.iter().take(10) {
            log(&format!("{}", byte));
        }

        // TODO: figure out how to use InMemNiftiObject correctly
        // because the data gets destroyed at this point
        let obj = InMemNiftiObject::from_reader(raw_data).unwrap();
        // use obj
        let header = obj.header();
        let raw_pixels = obj.into_volume().into_ndarray::<u8>().unwrap();

        log(&format!("Rust data"));
        for elem in raw_pixels.iter().take(10) {
            log(&format!("{}", elem));
        }

        let max = raw_pixels.max().unwrap();

        // get a slice to see what's up
        let slice = raw_pixels
            .slice(s![.., .., 150])
            .to_slice()
            .iter()
            // JS needs 4 channels, RGBA -> repeat each element of vector
            .flat_map(|&x| {
                let scaled_x = x[0] / max * 255_u8 / 3_u8;
                iter::repeat(scaled_x).take(4)
            })
            // scale up alpha to max
            .enumerate()
            .map(|(i, x)| if i > 0 && (i + 1) % 4 == 0 { 255_u8 } else { x })
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
            width: raw_pixels.shape()[0] as u32,
            height: raw_pixels.shape()[1] as u32,
            raw_pixels: slice,
        }
    }

    pub fn render_to_canvas(&self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut self.raw_pixels.clone()),
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
    */
}

// Read image from 2D canvas
#[wasm_bindgen]
pub fn get_image_data(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> ImageData {
    let width = canvas.width();
    let height = canvas.height();

    ctx.get_image_data(0.0, 0.0, width as f64, height as f64)
        .unwrap()
}

// Place Lightbox onto 2D canvas
#[wasm_bindgen]
pub fn putImageData(
    canvas: &HtmlCanvasElement,
    ctx: &CanvasRenderingContext2d,
    mut new_image: Lightbox,
) {
    // Convert raw pixels into an ImageData object
    let new_img_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut new_image.raw_pixels),
        canvas.width(),
        canvas.height(),
    );

    // Place new imagedata onto canvas
    ctx.put_image_data(&new_img_data.unwrap(), 0.0, 0.0)
        .expect("Should put image data on canvas");
}

// Convert ImageData to a raw pixel vec of u8
#[wasm_bindgen]
pub fn to_raw_pixels(imgdata: ImageData) -> Vec<u8> {
    imgdata.data().to_vec()
}

// Convert HTML5 Canvas to Lightbox
#[wasm_bindgen]
#[no_mangle]
pub fn open_image(canvas: HtmlCanvasElement, ctx: CanvasRenderingContext2d) -> Lightbox {
    let imgdata = get_image_data(&canvas, &ctx);
    let raw_pixels = to_raw_pixels(imgdata);
    Lightbox {
        raw_pixels,
        width: canvas.width(),
        height: canvas.height(),
    }
}

#[wasm_bindgen]
pub fn rev(lb: &mut Lightbox) {
    lb.raw_pixels.reverse();
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
