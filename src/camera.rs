

use super::cgmath::{Vector3, Matrix4, EuclideanSpace, Point3, Deg, Angle, InnerSpace};

pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT
}

// Default values
const SPEED: f32 = 2.5;
const SENSITIVITY : f32 = 0.1;
const ZOOM : f32 = 45.0;

pub struct Camera {
    position : Vector3<f32>,
    front : Vector3<f32>,
    up : Vector3<f32>,
    right : Vector3<f32>,
    world_up : Vector3<f32>,

    yaw: f32,
    pitch: f32,

    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32
}

impl Camera {
    pub fn new(position: Vector3<f32>, world_up: Vector3<f32>, yaw: f32, pitch: f32) -> Camera {
        let mut c = Camera {
            position,
            front: Vector3{x: 0.0, y: 0.0, z: 0.0},
            up: Vector3{x: 0.0, y: 0.0, z: 0.0},
            right: Vector3{x: 0.0, y: 0.0, z: 0.0},
            world_up,

            yaw,
            pitch,
            
            movement_speed: SPEED,
            mouse_sensitivity: SENSITIVITY,
            zoom: ZOOM,
        };
        c.update_vectors();
        c
    }

    pub fn get_view_matrix(&self) -> Matrix4<f32> {
        return Matrix4::look_at(
            Point3::from_vec(self.position),
            Point3::from_vec(self.position + self.front),
            self.up
        );
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn get_position(&self) -> Vector3<f32> {
        return self.position;
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
            let velocity = self.movement_speed * delta_time;
            self.position += match direction {
                CameraMovement::FORWARD => self.front * velocity,
                CameraMovement::BACKWARD => -self.front * velocity,
                CameraMovement::LEFT => -self.right * velocity,
                CameraMovement::RIGHT => self.right * velocity
            };
    }

    pub fn process_mouse_movement(&mut self, xoffset: f32, yoffset: f32) {
        let xoffset = xoffset * self.mouse_sensitivity;
        let yoffset = yoffset * self.mouse_sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        self.pitch = self.pitch.min(89.0);
        self.pitch = self.pitch.max(-89.0);

        self.update_vectors();
    }

    pub fn process_mouse_scroll(&mut self, yoffset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= yoffset;
        }
        self.zoom = self.zoom.max(1.0);
        self.zoom = self.zoom.min(45.0);
    }

    fn update_vectors(&mut self) {
        // Update front vector
        self.front = Vector3{
            x: Deg(self.pitch).cos() * Deg(self.yaw).cos(),
            y: Deg(self.pitch).sin(),
            z: Deg(self.pitch).cos() * Deg(self.yaw).sin()
        }.normalize();

        // Update other vectors
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}