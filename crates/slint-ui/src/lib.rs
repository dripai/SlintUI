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
    fn implemented_component_specs_follow_completion_template() {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let spec_root = crate_root.join("../../design/component-specs");
        let specs = [
            "button.md",
            "card.md",
            "checkbox.md",
            "divider.md",
            "empty-state.md",
            "focus-ring.md",
            "form-row.md",
            "icon-button.md",
            "icon.md",
            "label.md",
            "modal-dialog.md",
            "confirm-dialog.md",
            "context-menu.md",
            "form.md",
            "overlay.md",
            "popup-menu.md",
            "progress.md",
            "scroll-area.md",
            "segmented-control.md",
            "select.md",
            "settings-section.md",
            "space.md",
            "spinner.md",
            "stack.md",
            "status-bar.md",
            "split-pane.md",
            "surface.md",
            "switch.md",
            "text-field.md",
            "theme.md",
            "toolbar.md",
            "tool-button.md",
            "tooltip.md",
            "tabs.md",
            "table.md",
            "toast.md",
            "tree.md",
            "typography.md",
            "split-button.md",
            "breadcrumb.md",
            "steps.md",
            "pagination.md",
            "navigation-rail.md",
            "command-palette.md",
            "multi-select.md",
            "auto-complete.md",
            "date-picker.md",
            "time-picker.md",
            "color-picker.md",
            "avatar.md",
            "data-grid.md",
            "statistic.md",
            "timeline.md",
            "image-view.md",
            "calendar.md",
            "notification.md",
            "drawer.md",
            "skeleton.md",
        ];
        let required_sections = [
            "状态：已实现",
            "## 用途与边界",
            "## 公开 API",
            "## 状态",
            "无障碍",
            "Gallery",
            "测试",
            "限制",
        ];

        for spec in specs {
            let text = fs::read_to_string(spec_root.join(spec)).expect("read component spec");
            for required in required_sections {
                assert!(
                    text.contains(required),
                    "{spec} is missing completion evidence: {required}"
                );
            }
        }
    }
}
