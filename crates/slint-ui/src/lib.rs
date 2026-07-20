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
    use std::fs;

    #[test]
    fn library_entry_point_exists() {
        assert!(slint_library_path().join("index.slint").is_file());
    }

    #[test]
    fn generated_icon_catalog_is_complete() {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let asset_root = crate_root.join("assets/icons/ant-design");
        let module_root = crate_root.join("ui/icons");
        assert!(module_root.join("catalog.slint").is_file());

        for (style, expected_count) in [("outlined", 447), ("filled", 249)] {
            let assets = fs::read_dir(asset_root.join(style))
                .expect("read icon asset directory")
                .filter_map(Result::ok)
                .filter(|entry| entry.path().extension().is_some_and(|value| value == "svg"))
                .collect::<Vec<_>>();
            assert_eq!(assets.len(), expected_count, "{style} asset count");

            for asset in &assets {
                let module = module_root
                    .join(style)
                    .join(asset.path().with_extension("slint").file_name().unwrap());
                assert!(
                    module.is_file(),
                    "missing module for {}",
                    asset.path().display()
                );
            }

            let aggregate = fs::read_to_string(module_root.join(format!("{style}.slint")))
                .expect("read aggregate icon module");
            assert_eq!(aggregate.lines().count(), expected_count, "{style} exports");
        }

        let manifest =
            fs::read_to_string(asset_root.join("manifest.json")).expect("read icon manifest");
        assert!(manifest.contains("\"version\": \"4.5.0\""));
        assert!(manifest.contains("\"outlined\": {"));
        assert!(manifest.contains("\"filled\": {"));
    }

    #[test]
    fn component_state_icons_do_not_use_font_glyphs() {
        let ui_root = slint_library_path();
        let checkbox =
            fs::read_to_string(ui_root.join("controls/checkbox.slint")).expect("read checkbox");
        let text_field =
            fs::read_to_string(ui_root.join("controls/text-field.slint")).expect("read text field");

        assert!(checkbox.contains("outlined/check.svg"));
        assert!(checkbox.contains("outlined/minus.svg"));
        assert!(!checkbox.contains('✓'));
        assert!(!checkbox.contains('−'));
        assert!(text_field.contains("outlined/close.svg"));
        assert!(!text_field.contains('×'));
    }

    #[test]
    fn public_component_specs_follow_contract_template() {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let spec_root = crate_root.join("../../design/component-specs");
        let mut specs = Vec::new();
        for category in fs::read_dir(&spec_root).expect("read component spec root") {
            let category = category.expect("read component spec category");
            if !category.path().is_dir() {
                continue;
            }
            for spec in fs::read_dir(category.path()).expect("read component spec category") {
                let spec = spec.expect("read component spec").path();
                if spec.extension().is_some_and(|extension| extension == "md") {
                    specs.push(spec);
                }
            }
        }
        specs.sort();
        assert_eq!(specs.len(), 89, "public component and global spec count");
        let required_sections = [
            "成熟度：`Alpha`",
            "## 公开 API",
            "## 视觉规范",
            "## 行为规范",
        ];

        for spec in specs {
            let text = fs::read_to_string(&spec).expect("read component spec");
            for required in required_sections {
                assert!(
                    text.contains(required),
                    "{} is missing component contract section: {required}",
                    spec.display()
                );
            }
            assert_eq!(
                text.lines().filter(|line| line.starts_with("## ")).count(),
                3,
                "{} must contain exactly the three component contract sections",
                spec.display()
            );
        }
    }
}
