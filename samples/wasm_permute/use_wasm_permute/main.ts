import * as wasm from "../pkg/wasm_permute.js";

// Learn more at https://docs.deno.com/runtime/manual/examples/module_metadata#concepts
if (import.meta.main) {
  const nums = new Int32Array([1, 2, 3, 4]);
  console.log(wasm.permute(nums));
}
