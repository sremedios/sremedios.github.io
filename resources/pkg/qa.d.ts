/* tslint:disable */
/* eslint-disable */
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {ImageData}
*/
export function get_image_data(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D): ImageData;
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @param {Lightbox} new_image
*/
export function putImageData(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D, new_image: Lightbox): void;
/**
* @param {ImageData} imgdata
* @returns {Uint8Array}
*/
export function to_raw_pixels(imgdata: ImageData): Uint8Array;
/**
* @param {HTMLCanvasElement} canvas
* @param {CanvasRenderingContext2D} ctx
* @returns {Lightbox}
*/
export function open_image(canvas: HTMLCanvasElement, ctx: CanvasRenderingContext2D): Lightbox;
/**
* @param {Lightbox} lb
*/
export function rev(lb: Lightbox): void;
/**
*/
export class Lightbox {
  free(): void;
/**
* @param {Uint8Array} raw_pixels
* @param {number} width
* @param {number} height
* @returns {Lightbox}
*/
  static new(raw_pixels: Uint8Array, width: number, height: number): Lightbox;
}
