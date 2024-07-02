use typst::eval::Tracer;
use wasm_minimal_protocol::*;

use crate::io::{Input, Output};
use crate::world::MatryoshkaWorld;

mod font;
mod io;
mod world;

initiate_protocol!();

#[wasm_func]
pub fn compile(byte_source: &[u8]) -> Result<Vec<u8>, String> {
    let input: Input = match ciborium::from_reader(byte_source) {
        Ok(i) => i,
        Err(err) => return Err(format!("Failed to parse input {}", err))
    };

    let world = MatryoshkaWorld::new(input.source, input.filesystem);

    let mut tracer = Tracer::new();
    let result = typst::compile(&world, &mut tracer);

    let output = match result {
        Ok(document) => {
            let svg = document
                .pages
                .into_iter()
                .map(|page| {
                    typst_svg::svg(&page.frame)
                })
                .collect();

            Output::new(svg)
        }
        Err(errors) => {
            let err = errors.into_iter().map(|err| {
                    err.message
                })
                .fold(String::new(), |a, b| a + b.as_ref() + "\n");

            if input.dont_fail {
                Output::error(err)
            } else {
                return Err(err)
            }
        }
    };

    let mut bytes_output = Vec::new();
    match ciborium::into_writer(&output, &mut bytes_output) {
        Ok(()) => {},
        Err(err) => return Err(format!("Failed to encode output: {}", err))
    };

    Ok(bytes_output)
}
