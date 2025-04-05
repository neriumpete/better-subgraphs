/* tslint:disable */
/* eslint-disable */
export function init(): void;
export class Subgraph {
  free(): void;
  constructor();
  handle_event(event_json: string): void;
  get_gravatar(id: string): string;
  get_gravatars_by_owner(owner: string): string;
  get_last_processed_block(): bigint;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_subgraph_free: (a: number, b: number) => void;
  readonly subgraph_new: () => number;
  readonly subgraph_handle_event: (a: number, b: number, c: number) => [number, number];
  readonly subgraph_get_gravatar: (a: number, b: number, c: number) => [number, number, number, number];
  readonly subgraph_get_gravatars_by_owner: (a: number, b: number, c: number) => [number, number, number, number];
  readonly subgraph_get_last_processed_block: (a: number) => bigint;
  readonly init: () => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
