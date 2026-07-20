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
            "--section" => gallery.set_section(value.parse().unwrap_or(0)),
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
            "--icon-style" => {
                let style = if value == "filled" { 1 } else { 0 };
                gallery.set_icon_style(style);
                gallery.set_section(style);
            }
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
    fn implemented_controls_change_state_once_per_activation() {
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

        harness.invoke_select_p1_tab(1);
        assert_eq!(harness.get_p1_tab_index(), 1);
        assert_eq!(harness.get_p1_tab_activations(), 1);
        harness.invoke_select_p1_tab(1);
        harness.invoke_select_p1_tab(2);
        assert_eq!(harness.get_p1_tab_activations(), 1);

        harness.invoke_select_p1_tree(1);
        assert_eq!(harness.get_p1_tree_index(), 1);
        assert!(!harness.get_p1_tree_expanded());
        harness.invoke_toggle_p1_tree(0);
        assert!(harness.get_p1_tree_expanded());
        assert_eq!(harness.get_p1_tree_toggles(), 1);

        harness.invoke_set_p1_split_ratio(0.75);
        assert_eq!(harness.get_p1_split_ratio(), 0.75);
        assert_eq!(harness.get_p1_split_changes(), 1);

        harness.invoke_activate_p1_menu(0);
        harness.invoke_activate_p1_menu(1);
        assert_eq!(harness.get_p1_menu_selections(), 1);

        harness.invoke_submit_invalid_p1_form();
        assert_eq!(harness.get_p1_form_invalid_submits(), 1);

        harness.invoke_activate_p2_split();
        assert_eq!(harness.get_p2_split_activations(), 1);
        harness.invoke_activate_p2_split_menu(0);
        harness.invoke_activate_p2_split_menu(1);
        assert_eq!(harness.get_p2_menu_activations(), 1);

        harness.invoke_select_p2_breadcrumb(0);
        harness.invoke_select_p2_breadcrumb(1);
        assert_eq!(harness.get_p2_breadcrumb_activations(), 1);

        harness.invoke_select_p2_step(1);
        harness.invoke_select_p2_step(2);
        assert_eq!(harness.get_p2_step_index(), 1);
        assert_eq!(harness.get_p2_step_activations(), 1);

        harness.invoke_select_p2_page(3);
        harness.invoke_select_p2_page(99);
        assert_eq!(harness.get_p2_page(), 5);
        assert_eq!(harness.get_p2_page_activations(), 2);

        harness.invoke_select_p2_rail(1);
        harness.invoke_select_p2_rail(2);
        assert_eq!(harness.get_p2_rail_index(), 1);
        assert_eq!(harness.get_p2_rail_activations(), 1);

        harness.invoke_activate_p2_command(0);
        harness.invoke_activate_p2_command(1);
        assert_eq!(harness.get_p2_command_activations(), 1);

        harness.invoke_request_p2_multi(0, true);
        harness.invoke_request_p2_multi(1, true);
        harness.invoke_request_p2_multi(99, true);
        assert_eq!(harness.get_p2_multi_activations(), 1);

        harness.invoke_choose_p2_auto(0);
        harness.invoke_choose_p2_auto(1);
        assert_eq!(harness.get_p2_auto_index(), 0);
        assert_eq!(harness.get_p2_auto_value(), "Slint");
        assert_eq!(harness.get_p2_auto_activations(), 1);

        harness.invoke_select_p2_date(2027, 12, 31);
        harness.invoke_select_p2_date(2027, 13, 1);
        assert_eq!(harness.get_p2_date_year(), 2027);
        assert_eq!(harness.get_p2_date_month(), 12);
        assert_eq!(harness.get_p2_date_day(), 31);
        assert_eq!(harness.get_p2_date_activations(), 1);

        harness.invoke_select_p2_time(23, 59, 58);
        harness.invoke_select_p2_time(24, 0, 0);
        assert_eq!(harness.get_p2_time_hour(), 23);
        assert_eq!(harness.get_p2_time_minute(), 59);
        assert_eq!(harness.get_p2_time_activations(), 1);

        harness.invoke_select_p2_color(0);
        harness.invoke_select_p2_color(1);
        assert_eq!(harness.get_p2_color_activations(), 1);

        harness.invoke_request_p2_range(90, 20);
        harness.invoke_request_p2_range(100, 20);
        assert_eq!(harness.get_p2_range_requests(), 1);
        harness.invoke_request_p2_edit(20, 0);
        harness.invoke_request_p2_edit(20, 3);
        assert_eq!(harness.get_p2_edit_requests(), 1);

        harness.invoke_select_p2_calendar(0);
        harness.invoke_select_p2_calendar(1);
        assert_eq!(harness.get_p2_calendar_activations(), 1);
    }
}
