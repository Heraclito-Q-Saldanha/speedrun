use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sha256(input: String) -> String {
	let mut hasher = Sha256::new();

	hasher.update(input.as_bytes());

	let output = hasher.finalize();
	format!("{:x}", output)
}
