/* tslint:disable */
/* eslint-disable */
/**
* verify proofs from strings of decimals
* @param {Array<any>} vk_a
* @param {Array<any>} vk_b
* @param {Array<any>} vk_g
* @param {Array<any>} vk_d
* @param {Array<any>} vk_g_abc
* @param {Array<any>} p_a
* @param {Array<any>} p_b
* @param {Array<any>} p_c
* @param {Array<any>} pub_f
* @returns {boolean}
*/
export function verify_external(vk_a: Array<any>, vk_b: Array<any>, vk_g: Array<any>, vk_d: Array<any>, vk_g_abc: Array<any>, p_a: Array<any>, p_b: Array<any>, p_c: Array<any>, pub_f: Array<any>): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly verify_external: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
  readonly __wbindgen_export_0: (a: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => number;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
