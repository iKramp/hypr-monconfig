use clay_layout::{Declaration, fixed};
use raylib::texture::Texture2D;

#[rustfmt::skip]
pub fn draw_monitor_area<'a>(_scale: f32, _position: (f32, f32)) -> Declaration<'a, Texture2D, ()> {
    let mut declaration = Declaration::new();
    declaration
        .layout()
            .width(fixed!(100.))
            .height(fixed!(100.))
        .end()
        .corner_radius()
            .all(24.)
        .end()
        .background_color((0x50, 0x50, 0x50).into());

    declaration
}
