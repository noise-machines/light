use std::path::Path;

pub fn folder() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
