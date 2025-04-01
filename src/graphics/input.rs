use crate::object_loader::Object;

use super::{Camera, Light, RenderContext};
use std::{f32::consts, time::Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::{Key, NamedKey},
};

pub struct InputState {
    pub window_size: [f32; 2],
    pub mouse_pos: [f32; 2],
    pub mouse_delta: [f32; 2],
    pub mouse_scroll_delta: f32,
    pub mouse_left_click: bool,
    pub mouse_right_click: bool,
    pub btn_zoom_in: bool,
    pub btn_zoom_out: bool,
    pub btn_rotate_left: bool,
    pub btn_rotate_right: bool,
    pub btn_move_up: bool,
    pub btn_move_down: bool,
    pub btn_lock_light: bool,
    pub btn_light_color: bool,
    pub btn_texture: bool,
    pub btn_reset: bool,
    pub btn_quit: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            window_size: [0.0, 0.0],
            mouse_pos: [0.0, 0.0],
            mouse_delta: [0.0, 0.0],
            mouse_scroll_delta: 0.0,
            mouse_left_click: false,
            mouse_right_click: false,
            btn_zoom_in: false,
            btn_zoom_out: false,
            btn_rotate_left: false,
            btn_rotate_right: false,
            btn_move_up: false,
            btn_move_down: false,
            btn_lock_light: false,
            btn_light_color: false,
            btn_texture: false,
            btn_reset: false,
            btn_quit: false,
        }
    }

    pub fn handle_input(&mut self, window_size: PhysicalSize<u32>, event: &WindowEvent) {
        self.window_size = window_size.into();

        match event {
            WindowEvent::KeyboardInput { event, .. } => self.on_keyboard_event(event),
            WindowEvent::MouseInput { state, button, .. } => {
                self.on_mouse_click_event(*state, *button)
            }
            WindowEvent::CursorMoved { position, .. } => self.on_cursor_moved_event(position),
            WindowEvent::MouseWheel { delta, .. } => self.on_mouse_wheel_event(delta),
            _ => {}
        }
    }

    fn on_keyboard_event(&mut self, event: &KeyEvent) {
        match event.logical_key.as_ref() {
            Key::Character("w") => self.btn_zoom_in = event.state.is_pressed(),
            Key::Character("s") => self.btn_zoom_out = event.state.is_pressed(),
            Key::Character("a") => self.btn_rotate_left = event.state.is_pressed(),
            Key::Character("d") => self.btn_rotate_right = event.state.is_pressed(),
            Key::Character("l") => self.btn_lock_light = event.state.is_pressed(),
            Key::Character("c") => self.btn_light_color = event.state.is_pressed(),
            Key::Character("r") => self.btn_reset = event.state.is_pressed(),
            Key::Character("t") => self.btn_texture = event.state.is_pressed(),
            Key::Named(NamedKey::Space) => self.btn_move_up = event.state.is_pressed(),
            Key::Named(NamedKey::Shift) => self.btn_move_down = event.state.is_pressed(),
            Key::Named(NamedKey::Escape) => self.btn_quit = event.state.is_pressed(),
            _ => {}
        }
    }

    fn on_mouse_click_event(&mut self, state: ElementState, mouse_btn: MouseButton) {
        match mouse_btn {
            MouseButton::Left => self.mouse_left_click = state.is_pressed(),
            MouseButton::Right => self.mouse_right_click = state.is_pressed(),
            _ => {}
        }
    }

    fn on_mouse_wheel_event(&mut self, delta: &MouseScrollDelta) {
        let change = match delta {
            MouseScrollDelta::LineDelta(_x, y) => *y,
            MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
        };
        self.mouse_scroll_delta += change;
    }

    fn on_cursor_moved_event(&mut self, pos: &PhysicalPosition<f64>) {
        let normalized_pos = [
            (pos.x as f32 / self.window_size[0]).clamp(0.0, 1.0),
            (pos.y as f32 / self.window_size[1]).clamp(0.0, 1.0),
        ];
        self.mouse_delta = [
            (self.mouse_pos[0] - normalized_pos[0]),
            self.mouse_pos[1] - normalized_pos[1],
        ];
        self.mouse_pos = normalized_pos;
    }

    pub fn reset(&mut self) {
        self.mouse_delta = [0.0, 0.0];
        self.mouse_scroll_delta = 0.0;
        self.btn_lock_light = false;
        self.btn_light_color = false;
        self.btn_texture = false;
        self.btn_reset = false;
    }
}

impl RenderContext {
    pub fn update_state_after_inputs(&mut self, object: &Object) {
        let state = &self.input_state;
        let time = &self.time_info;
        let camera = &mut self.camera;
        let light = &mut self.light;

        let camera_speed = time.dt * object.size.length();

        if state.btn_zoom_in {
            camera.distance -= camera_speed;
            if camera.distance < 0.0 {
                camera.distance = 0.0;
            }
        }
        if state.btn_zoom_out {
            camera.distance += camera_speed;
        }
        if state.btn_rotate_left {
            camera.theta = (camera.theta - consts::PI * time.dt) % (2.0 * consts::PI);
        }
        if state.btn_rotate_right {
            camera.theta = (camera.theta + consts::PI * time.dt) % (2.0 * consts::PI);
        }
        if state.btn_move_up {
            camera.target.y += camera_speed;
        }
        if state.btn_move_down {
            camera.target.y -= camera_speed;
        }
        if state.mouse_left_click {
            camera.theta += -state.mouse_delta[0] * 10.0;
            camera.phi += -state.mouse_delta[1] * 10.0;
            camera.phi = f32::max(
                f32::min(camera.phi, consts::FRAC_PI_2 - 0.1),
                -consts::FRAC_PI_2 + 0.1,
            );
        }
        if state.mouse_scroll_delta != 0.0 {
            camera.distance += -state.mouse_scroll_delta;
        }
        if state.btn_lock_light {
            light.pos_locked = !light.pos_locked;
        }
        if state.btn_light_color {
            light.color.0 = (light.color.0 + 1) % light.colors.len();
        }
        if state.btn_texture {
            self.use_texture = !self.use_texture;
        }

        camera.update_position();
        if !light.pos_locked {
            light.position = camera.position;
        }

        if state.btn_reset {
            self.camera = Camera {
                target: object.center,
                distance: 5.0 * f32::max(object.size.x, f32::max(object.size.y, object.size.z)),
                ..Default::default()
            };
            self.light = Light::default();
        }
    }

    /// Returns the average FPS.
    pub fn avg_fps(&self) -> f32 {
        self.time_info.avg_fps
    }

    pub fn update_time(&mut self) {
        // Each second, update average fps & reset frame count & dt sum.
        let time = &mut self.time_info;

        if time.dt_sum > 1.0 {
            time.avg_fps = time.frame_count / time.dt_sum;
            time.frame_count = 0.0;
            time.dt_sum = 0.0;
        }
        time.dt = time.time.elapsed().as_secs_f32();
        time.dt_sum += time.dt;
        time.frame_count += 1.0;
        time.time = Instant::now();
    }
}
