use std::collections::HashMap;

fn main() {
    let library_paths = HashMap::from([("slint-ui".to_owned(), slint_ui::slint_library_path())]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library_paths);

    slint_build::compile_with_config("ui/gallery.slint", config).expect("compile SlintUI Gallery");
}
