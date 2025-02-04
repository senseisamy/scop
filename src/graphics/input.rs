use super::RenderContext;
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
    }
}

const CAMERA_SPEED: f32 = 0.05;

impl RenderContext {
    pub fn update_state_after_inputs(&mut self) {
        let state = &self.input_state;

        if state.btn_zoom_in {
            self.camera.distance -= CAMERA_SPEED;
            if self.camera.distance < 0.0 {
                self.camera.distance = 0.0;
            }
        }
        if state.btn_zoom_out {
            self.camera.distance += CAMERA_SPEED;
        }
        if state.btn_rotate_left {
            self.camera.theta = (self.camera.theta + CAMERA_SPEED) % (2.0 * consts::PI);
        }
        if state.btn_rotate_right {
            self.camera.theta = (self.camera.theta - CAMERA_SPEED) % (2.0 * consts::PI);
        }
        if state.btn_move_up {
            self.camera.target.y += CAMERA_SPEED;
        }
        if state.btn_move_down {
            self.camera.target.y -= CAMERA_SPEED;
        }
        if state.mouse_left_click {
            self.camera.theta += state.mouse_delta[0] * 5.0;
            self.camera.phi += -state.mouse_delta[1] * 5.0;
            self.camera.phi = f32::max(
                f32::min(self.camera.phi, consts::FRAC_PI_2 - 0.1),
                -consts::FRAC_PI_2 + 0.1,
            );
        }
        // if state.right_click {
        //     let rotate_y = Mat4::rotate_y(-state.mouse_delta[0] * 2.0);

        //     self.camera.direction *= rotate_y;
        // }
        if state.mouse_scroll_delta != 0.0 {
            self.camera.distance += -state.mouse_scroll_delta;
        }
        self.camera.update_position();
    }

    /// Returns the average FPS.
    pub fn avg_fps(&self) -> f32 {
        self.avg_fps
    }

    pub fn update_time(&mut self) {
        // Each second, update average fps & reset frame count & dt sum.
        if self.dt_sum > 1.0 {
            self.avg_fps = self.frame_count / self.dt_sum;
            self.frame_count = 0.0;
            self.dt_sum = 0.0;
        }
        self.dt = self.time.elapsed().as_secs_f32();
        self.dt_sum += self.dt;
        self.frame_count += 1.0;
        self.time = Instant::now();
    }
}
