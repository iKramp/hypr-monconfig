use clay_layout::{Declaration, grow, percent};
use raylib::texture::Texture2D;

pub mod monitor_area;

struct DrawUiInfo {
    // Add fields as needed
}

pub fn draw_ui() {}

#[rustfmt::skip]
pub fn section_main_area() -> (Declaration<'static, Texture2D, ()>, Declaration<'static, Texture2D, ()>) {
    let mut monitor_declaration = Declaration::new();
    monitor_declaration
        .layout()
            .width(grow!())
            .height(percent!(0.5))
        .end()
        .corner_radius()
            .all(24.)
        .end()
        .background_color((90, 85, 181).into()); //blue

    let mut settings_declaration = Declaration::new();
    settings_declaration
        .layout()
            .width(grow!())
            .height(percent!(0.5))
        .end()
        .corner_radius()
            .all(24.)
        .end()
        .background_color((85, 181, 128).into()); //green
    (monitor_declaration, settings_declaration)
}
