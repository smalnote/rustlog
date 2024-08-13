/*
 * Cargo - Official package manager and building tool
 *   - One `Cargo.lock`, `Cargo.toml` for dependencies and configuration
 * Package/Project - cargo project unit
 * Crate - Compilation unit os Rust source code
 *   - Library crate: a package can have at most one library crate
 *   - Binary crate: a package can have multiple binary crates
 *   - src/main.rs: default binary crate root
 *       more binary crates can be placed in src/bin/[binary_crate]/main.rs or src/bin/[binary_crate].rs
 *   - src/lib.rs: default library crate root
 *   - Crate root: Rust compiler starts from the crate root, modules should be
 *     included in crate root to compile
 * Modules - `mod` is the short for module
 *   - file module: src/[module_name].rs means mod [module_name]
 *   - directory module: src/[module_name]/mod.rs also means mod [module_name]
 *   - inline module: code `mod [module_name] {}` means submodule of the module specified by above
 *   - code `pub mod [module_name]` exports module, eventually available in crate root
 *
 */
