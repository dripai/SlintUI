use std::{fs::File, io::BufWriter, path::Path, time::Duration};

use slint::ComponentHandle;

slint::include_modules!();

fn write_snapshot(gallery: &GalleryWindow, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let snapshot = gallery.window().take_snapshot()?;
    let file = BufWriter::new(File::create(path)?);
    let mut encoder = png::Encoder::new(file, snapshot.width(), snapshot.height());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()?
        .write_image_data(snapshot.as_bytes())?;
    Ok(())
}

fn run_bounded(
    gallery: &GalleryWindow,
    screenshot: Option<std::path::PathBuf>,
) -> Result<(), slint::PlatformError> {
    gallery.show()?;
    let weak = gallery.as_weak();
    // Give the software renderer enough time to paint resource-heavy catalog pages.
    slint::Timer::single_shot(Duration::from_millis(2_000), move || {
        if let (Some(gallery), Some(path)) = (weak.upgrade(), screenshot.as_deref())
            && let Err(error) = write_snapshot(&gallery, path)
        {
            eprintln!("failed to write Gallery snapshot: {error}");
        }
        let _ = slint::quit_event_loop();
    });
    slint::run_event_loop()
}

fn apply_options(gallery: &GalleryWindow, args: &mut impl Iterator<Item = std::ffi::OsString>) {
    while let Some(flag) = args.next() {
        let Some(flag) = flag.to_str() else { continue };
        let Some(value) = args.next() else { break };
        let Some(value) = value.to_str() else {
            continue;
        };

        match flag {
            "--page" => gallery.set_page(value.parse().unwrap_or(0)),
            "--theme" => gallery.set_theme_mode(match value {
                "dark" => ThemeMode::Dark,
                "high-contrast" => ThemeMode::HighContrast,
                _ => ThemeMode::Light,
            }),
            "--density" => gallery.set_density_mode(match value {
                "compact" => Density::Compact,
                "comfortable" => Density::Comfortable,
                _ => Density::Regular,
            }),
            "--scale" => gallery.set_preview_scale(value.parse().unwrap_or(1.0)),
            "--icon-style" => gallery.set_icon_style(if value == "filled" { 1 } else { 0 }),
            "--locale" => {
                gallery.set_locale(value.into());
                gallery.set_layout_direction(if value.starts_with("ar") {
                    Direction::Rtl
                } else {
                    Direction::Ltr
                });
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let mut args = std::env::args_os().skip(1);
    let command = args.next();
    let gallery = GalleryWindow::new()?;

    match command.as_deref().and_then(|value| value.to_str()) {
        Some("--smoke-test") => {
            apply_options(&gallery, &mut args);
            run_bounded(&gallery, None)
        }
        Some("--screenshot") => {
            let path = args.next().unwrap_or_else(|| "gallery.png".into());
            apply_options(&gallery, &mut args);
            gallery.set_reduced_motion(true);
            run_bounded(&gallery, Some(path.into()))
        }
        _ => gallery.run(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p0_controls_change_state_once_per_activation() {
        let harness = InteractionHarness::new().expect("create interaction harness");

        assert_eq!(harness.get_button_activations(), 0);
        harness.invoke_activate_button();
        assert_eq!(harness.get_button_activations(), 1);

        harness.invoke_activate_disabled_button();
        assert_eq!(harness.get_disabled_button_activations(), 0);

        harness.invoke_activate_icon_button();
        assert_eq!(harness.get_icon_button_activations(), 1);

        assert!(!harness.get_tool_button_checked());
        harness.invoke_activate_tool_button();
        assert!(harness.get_tool_button_checked());
        assert_eq!(harness.get_tool_button_activations(), 1);

        assert!(!harness.get_checkbox_checked());
        harness.invoke_toggle_checkbox();
        assert!(harness.get_checkbox_checked());

        assert!(!harness.get_disabled_checkbox_checked());
        harness.invoke_toggle_disabled_checkbox();
        assert!(!harness.get_disabled_checkbox_checked());

        assert!(!harness.get_switch_checked());
        harness.invoke_toggle_switch();
        assert!(harness.get_switch_checked());

        assert!(!harness.get_disabled_switch_checked());
        harness.invoke_toggle_disabled_switch();
        assert!(!harness.get_disabled_switch_checked());

        assert_eq!(harness.get_segmented_index(), 0);
        harness.invoke_select_segmented(1);
        assert_eq!(harness.get_segmented_index(), 1);
        assert_eq!(harness.get_segmented_value(), "Two");
        assert_eq!(harness.get_segmented_activations(), 1);
        harness.invoke_select_segmented(1);
        harness.invoke_select_segmented(99);
        assert_eq!(harness.get_segmented_activations(), 1);

        assert_eq!(harness.get_text_field_value(), "Initial");
        harness.invoke_clear_text_field();
        assert_eq!(harness.get_text_field_value(), "");
        assert_eq!(harness.get_text_field_edits(), 1);
        assert_eq!(harness.get_text_field_clears(), 1);
        harness.invoke_clear_text_field();
        assert_eq!(harness.get_text_field_clears(), 1);
        harness.invoke_clear_disabled_text_field();
        assert_eq!(harness.get_disabled_text_field_value(), "Locked");

        assert_eq!(harness.get_select_index(), 0);
        harness.invoke_select_option(2);
        assert_eq!(harness.get_select_index(), 2);
        assert_eq!(harness.get_select_value(), "Gamma");
        assert_eq!(harness.get_select_activations(), 1);
        harness.invoke_select_option(2);
        harness.invoke_select_option(-1);
        assert_eq!(harness.get_select_activations(), 1);
        harness.invoke_select_disabled_option(1);
        assert_eq!(harness.get_disabled_select_index(), 0);
    }
}
