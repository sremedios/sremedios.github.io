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
export function main(): void;
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

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_lightbox_free: (a: number) => void;
  readonly lightbox_new: (a: number, b: number, c: number, d: number) => number;
  readonly get_image_data: (a: number, b: number) => number;
  readonly putImageData: (a: number, b: number, c: number) => void;
  readonly to_raw_pixels: (a: number, b: number) => void;
  readonly open_image: (a: number, b: number) => number;
  readonly rev: (a: number) => void;
  readonly main: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        