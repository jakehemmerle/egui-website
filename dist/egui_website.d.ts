/* tslint:disable */
/* eslint-disable */
/**
*/
export class WebHandle {
  free(): void;
/**
*/
  constructor();
/**
* @param {string} canvas_id
* @returns {Promise<void>}
*/
  start(canvas_id: string): Promise<void>;
/**
*/
  destroy(): void;
/**
* @returns {boolean}
*/
  has_panicked(): boolean;
/**
* @returns {string | undefined}
*/
  panic_message(): string | undefined;
/**
* @returns {string | undefined}
*/
  panic_callstack(): string | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_webhandle_free: (a: number) => void;
  readonly webhandle_new: () => number;
  readonly webhandle_start: (a: number, b: number, c: number) => number;
  readonly webhandle_destroy: (a: number) => void;
  readonly webhandle_has_panicked: (a: number) => number;
  readonly webhandle_panic_message: (a: number, b: number) => void;
  readonly webhandle_panic_callstack: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h25bec0b7be9cbd6b: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h5e630810dde13afc: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc32362a6090edd3e: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h92952546dd435fbd: (a: number, b: number, c: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__hae4beb0581cae068: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
