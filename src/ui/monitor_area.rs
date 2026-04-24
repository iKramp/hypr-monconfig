use clay_layout::{ClayLayoutScope, Declaration, grow, layout::LayoutDirection};
use raylib::texture::Texture2D;

#[rustfmt::skip]
pub fn draw_monitor_area<'a>(scale: f32, position: (f32, f32)) -> Declaration<'a, Texture2D, ()> {
    let mut declaration = Declaration::new();
    declaration
        .layout()
            .width(grow!())
            .height(grow!())
            .direction(LayoutDirection::TopToBottom)
        .end()
        .corner_radius()
            .all(24.)
        .end()
        .background_color((0x50, 0x50, 0x50).into());

    declaration
}
