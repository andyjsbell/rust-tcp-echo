extern crate wasm_bindgen;
extern crate server;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_server() {
    println!("Does this log?");

    server::server::run();
}