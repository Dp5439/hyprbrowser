use iced::widget::*;
use iced::Element;

pub fn view(browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let theme_selector = row![
        text("Theme:").size(12),
        pick_list(
            vec!["Light", "Dark", "System"],
            Some("System"),
            |_| crate::Message::ThemeChanged(crate::theme::ThemeMode::System),
        )
        .padding(8),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center);

    let save_state_btn = button(text("SAVE"))
        .padding(8)
        .on_press(crate::Message::ToggleWorkflow);

    let restore_state_btn = button(text("RESTORE"))
        .padding(8)
        .on_press(crate::Message::ToggleWorkflow);

    // GPU Information Section
    let gpu_tier_text = format!("{:?}", browser.gpu_info.tier);
    let gpu_info = iced::widget::column![
        text("GPU Information").size(14),
        text(format!("Adapter: {}", browser.gpu_info.adapter_name)).size(10),
        text(format!("Tier: {}", gpu_tier_text)).size(10),
        text(format!(
            "Max Texture: {} | Max Buffer: {}",
            browser.gpu_info.max_texture_size, browser.gpu_info.max_uniform_buffer_size
        ))
        .size(10),
    ]
    .spacing(6)
    .padding(12);

    // GPU Settings Section
    let effect_intensity_text = format!(
        "Effect Intensity: {:.0}%",
        browser.gpu_settings.effect_intensity * 100.0
    );
    let effect_toggle = button(text(effect_intensity_text))
        .padding(8)
        .on_press(crate::Message::ToggleEffectIntensity);

    let particle_slider = row![
        text("Particle Count:").size(12),
        text(format!("{}", browser.gpu_settings.particle_count)).size(12),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center);

    let fps_target_text = format!("Target FPS: {} Hz", browser.gpu_settings.animation_fps_target);

    let reduce_transparency_toggle = row![
        text("Reduce Transparency:").size(12),
        checkbox("", browser.gpu_settings.reduce_transparency_effects)
            .on_toggle(|_| crate::Message::ToggleEffectIntensity),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center);

    let gpu_settings = iced::widget::column![
        text("GPU Performance Settings").size(14),
        effect_toggle,
        particle_slider,
        text(fps_target_text).size(11),
        reduce_transparency_toggle,
        text("Note: Adjust these settings if experiencing performance issues").size(9),
    ]
    .spacing(8)
    .padding(12);

    container(
        scrollable(
            iced::widget::column![
                text("Workflow Settings").size(16),
                theme_selector,
                row![save_state_btn, restore_state_btn].spacing(8),
                gpu_info,
                gpu_settings,
            ]
            .spacing(12)
            .padding(16),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
