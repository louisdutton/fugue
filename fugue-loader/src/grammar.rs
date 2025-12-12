// Copyright (c) 2020 Blaž Hrastnik
// Licensed under MPL-2.0

use anyhow::Result;
use std::path::PathBuf;
use tree_house::tree_sitter::Grammar;

#[cfg(unix)]
const DYLIB_EXTENSION: &str = "so";

#[cfg(target_arch = "wasm32")]
const DYLIB_EXTENSION: &str = "wasm";

#[cfg(target_arch = "wasm32")]
pub fn get_language(name: &str) -> Result<Option<Grammar>> {
    unimplemented!()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_language(name: &str) -> Result<Option<Grammar>> {
    let mut rel_library_path = PathBuf::new().join("grammars").join(name);
    rel_library_path.set_extension(DYLIB_EXTENSION);
    let library_path = crate::runtime_file(&rel_library_path);
    if !library_path.exists() {
        return Ok(None);
    }

    let grammar = unsafe { Grammar::new(name, &library_path) }?;
    Ok(Some(grammar))
}

/// Gives the contents of a file from a language's `runtime/queries/<lang>`
/// directory
pub fn load_runtime_file(language: &str, filename: &str) -> Result<String, std::io::Error> {
    let path = crate::runtime_file(PathBuf::new().join("queries").join(language).join(filename));
    std::fs::read_to_string(path)
}
