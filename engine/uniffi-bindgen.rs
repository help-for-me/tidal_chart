//! Generates Swift/Kotlin bindings from this crate's `#[uniffi::export]`
//! annotations. Not shipped — only built behind the `bindgen` feature.
//! Run with: `cargo run --features bindgen --bin uniffi-bindgen -- \
//!   generate --library <path-to-built-lib> --language swift --out-dir <dir>`

fn main() {
    uniffi::uniffi_bindgen_main()
}
