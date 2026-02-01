use egui::CursorIcon;
use egui_wgpu::Renderer;
use wgpu::InstanceDescriptor;

use std::sync::Arc;
use web_time::{Duration, Instant};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::*,
    window::{Theme, Window},
    event_loop::{ActiveEventLoop, EventLoop}, 
    keyboard::{KeyCode, PhysicalKey}
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_attributes = Window::default_attributes()
    .with_title("Tails v0.0.1");

    let main_window = Some(event_loop.create_window(window_attributes).unwrap());

    event_loop.run(move |event: Event<()>, event_loop| {
        

    });
}