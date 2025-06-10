wasm_pack: src/lib.rs
	wasm-pack build --target deno

use_wasm_permute: wasm_pack
	cd ./use_wasm_permute && deno run --allow-read main.ts
