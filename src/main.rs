use std::num::NonZero;

use softbuffer::{Context, Surface};
use tiny_skia::{Color, Paint, Pixmap};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App {
        window: None,
        size: (800, 600),
    };
    let _ = event_loop.run_app(&mut app);
}

struct App {
    size: (usize, usize),
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("size: {:?}", self.size);
        let mut attrs = Window::default_attributes();
        attrs = attrs
            .with_resizable(true)
            .with_min_inner_size(winit::dpi::LogicalSize::new(800, 450));
        attrs = attrs.with_inner_size(winit::dpi::LogicalSize::new(
            self.size.0 as u32,
            self.size.1 as u32,
        ));
        self.window = Some(event_loop.create_window(attrs).unwrap());
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let window = self.window.as_ref().unwrap();

                let size = window.inner_size();
                let width = size.width;
                let height = size.height;

                let context = Context::new(window).unwrap();
                let mut surface = Surface::new(&context, window).unwrap();
                surface
                    .resize(NonZero::new(width).unwrap(), NonZero::new(height).unwrap())
                    .unwrap();

                let mut pixmap = Pixmap::new(width, height).unwrap();
                let mut paint = Paint::default();

                paint.set_color_rgba8(100, 100, 100, 255);
                pixmap.fill(Color::from_rgba8(30, 30, 30, 255));

                // Copy pixmap data to window surface
                let mut buffer = surface.buffer_mut().unwrap();
                let buf = buffer.as_mut();
                let data = pixmap
                    .data()
                    .chunks_exact(4)
                    .map(|px| {
                        ((px[3] as u32) << 24)
                            | ((px[2] as u32) << 16)
                            | ((px[1] as u32) << 8)
                            | (px[0] as u32)
                    })
                    .collect::<Vec<_>>();
                for (dst, src) in buf.chunks_exact_mut(4).zip(data.chunks_exact(4)) {
                    dst.copy_from_slice(src);
                }
                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
