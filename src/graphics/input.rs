use std::time::Instant;

use winit::{dpi::PhysicalSize, event::{KeyEvent, WindowEvent}, keyboard::{Key, NamedKey}};

use crate::math::{Mat4, Vec3};

use super::RenderContext;

pub struct InputState {
    pub window_size: [f32; 2],
    pub move_forward: bool,
    pub move_backward: bool,
    pub move_left: bool,
    pub move_right: bool,
    pub move_down: bool,
    pub move_up: bool,
    pub cam_rotate_left: bool,
    pub cam_rotate_right: bool,
    pub cam_rotate_up: bool,
    pub cam_rotate_down: bool,
    pub world_rotate_left: bool,
    pub world_rotate_right: bool,
    pub should_quit: bool
}

impl InputState {
    pub fn new() -> Self {
        Self {
            window_size: [0.0, 0.0],
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            move_down: false,
            move_up: false,
            cam_rotate_left: false,
            cam_rotate_right: false,
            cam_rotate_up: false,
            cam_rotate_down: false,
            world_rotate_left: false,
            world_rotate_right: false,
            should_quit: false
        }
    }

    pub fn handle_input(&mut self, window_size: PhysicalSize<u32>, event: &WindowEvent) {
        self.window_size = window_size.into();

        match event {
            WindowEvent::KeyboardInput { event, .. } => self.on_keyboard_event(event),
            _ => {}
        }
    }

    pub fn on_keyboard_event(&mut self, event: &KeyEvent) {
        match event.logical_key.as_ref() {
            Key::Character("o") => self.world_rotate_left = event.state.is_pressed(),
            Key::Character("p") => self.world_rotate_right = event.state.is_pressed(),
            Key::Character("w") => self.move_forward = event.state.is_pressed(),
            Key::Character("s") => self.move_backward = event.state.is_pressed(),
            Key::Character("a") => self.move_left = event.state.is_pressed(),
            Key::Character("d") => self.move_right = event.state.is_pressed(),
            Key::Named(NamedKey::Space) => self.move_up = event.state.is_pressed(),
            Key::Named(NamedKey::Shift) => self.move_down = event.state.is_pressed(),
            Key::Named(NamedKey::ArrowUp) => self.cam_rotate_up = event.state.is_pressed(),
            Key::Named(NamedKey::ArrowDown) => self.cam_rotate_down = event.state.is_pressed(),
            Key::Named(NamedKey::ArrowRight) => self.cam_rotate_right = event.state.is_pressed(),
            Key::Named(NamedKey::ArrowLeft) => self.cam_rotate_left = event.state.is_pressed(),
            Key::Named(NamedKey::Escape) => self.should_quit = event.state.is_pressed(),
            _ => {}
        }
    }
}

const CAMERA_SPEED: f32 = 0.2;

impl RenderContext {
    pub fn update_state_after_inputs(&mut self) {
        if self.input_state.world_rotate_left {
            self.world.world_transformation *= Mat4::rotate_y(std::f32::consts::PI / 60.0);
        }
        if self.input_state.world_rotate_right {
            self.world.world_transformation *= Mat4::rotate_y(-std::f32::consts::PI / 60.0);
        }
        if self.input_state.move_forward {
            self.world.camera.position += self.world.camera.direction * -CAMERA_SPEED;
        }
        if self.input_state.move_backward {
            self.world.camera.position += self.world.camera.direction * CAMERA_SPEED;
        }
        if self.input_state.move_left {
            self.world.camera.position += Vec3::cross(
                &self.world.camera.direction,
                &Vec3::from(&[0.0, -1.0, 0.0]),
            ) * CAMERA_SPEED;
        }
        if self.input_state.move_right {
            self.world.camera.position += Vec3::cross(
                &self.world.camera.direction,
                &Vec3::from(&[0.0, -1.0, 0.0]),
            ) * -CAMERA_SPEED;
        }
        if self.input_state.move_up {
            self.world.camera.position.y += CAMERA_SPEED;
        }
        if self.input_state.move_down {
            self.world.camera.position.y -= CAMERA_SPEED;
        }
        if self.input_state.cam_rotate_left {
            self.world.camera.direction = self.world.camera.direction * Mat4::rotate_y(-std::f32::consts::PI / 120.0);
        }
        if self.input_state.cam_rotate_right {
            self.world.camera.direction = self.world.camera.direction * Mat4::rotate_y(std::f32::consts::PI / 120.0);
        }
        if self.input_state.cam_rotate_up {
            self.world.camera.direction = self.world.camera.direction * Mat4::rotate_x(std::f32::consts::PI / 120.0);
        }
        if self.input_state.cam_rotate_down {
            self.world.camera.direction = self.world.camera.direction * Mat4::rotate_x(-std::f32::consts::PI / 120.0);
        }
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