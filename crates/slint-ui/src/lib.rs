//! Build-time integration for the SlintUI component library.

use std::path::PathBuf;

/// Returns the directory registered as the `@slint-ui` Slint library path.
#[must_use]
pub fn slint_library_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_entry_point_exists() {
        assert!(slint_library_path().join("index.slint").is_file());
    }
}
