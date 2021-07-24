/* tslint:disable */
/* eslint-disable */
/**
* @param {number} lat
* @param {number} lon
* @param {Float64Array} distances
* @param {number} num_segments
* @returns {string}
*/
export function make_clockboard(lat: number, lon: number, distances: Float64Array, num_segments: number): string;
/**
* @param {number} n
* @returns {Float64Array}
*/
export function generate_triangular_sequence(n: number): Float64Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly make_clockboard: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly generate_triangular_sequence: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
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
