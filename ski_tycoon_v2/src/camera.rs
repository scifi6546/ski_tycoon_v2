use super::prelude::Terrain;
use legion::*;
use nalgebra::{Matrix4, Point, Vector2, Vector3};
pub struct DeltaCamera {
    previous: Camera,
    next: Option<Camera>,
}
impl DeltaCamera {
    //checks if next is used and if not creates next from previous
    fn new_next(&mut self) {
        if self.next.is_none() {
            self.next = Some(self.previous.clone());
        }
    }
    pub fn new(origin: Vector3<f32>, radius: f32, phi: f32, theta: f32) -> Self {
        Self {
            previous: Camera {
                origin,
                radius,
                phi,
                theta,
            },
            next: None,
        }
    }
    pub fn rotate_phi(&mut self, delta_phi: f32) {
        self.new_next();
        self.next.as_mut().unwrap().phi += delta_phi;
    }
    pub fn rotate_theta(&mut self, delta_theta: f32) {
        self.new_next();
        self.next.as_mut().unwrap().theta += delta_theta;
    }
    /// Increases by value proportional to delta radius
    pub fn update_radius(&mut self, delta_radius: f32) {
        self.new_next();
        let mut next = self.next.as_mut().unwrap();
        next.radius += delta_radius * next.radius;
        if next.radius > Camera::FAR_CLIP {
            next.radius = Camera::FAR_CLIP;
        }
        if next.radius < 0.1 {
            next.radius = 0.1;
        }
    }
    pub fn get_radius(&self) -> f32 {
        self.previous.radius
    }
    pub fn translate(&mut self, translation: &Vector3<f32>) {
        self.new_next();
        self.next.as_mut().unwrap().origin += translation;
    }
    /// applies delta changes to self
    pub fn apply(&mut self, world: &World) {
        if let Some(next) = self.next.as_ref() {
            let terrain = <&Terrain>::query().iter(world).next().unwrap();

            self.previous = next.clone();
            self.previous.origin.y = terrain
                .get_transform_rounded(&Vector2::new(
                    self.previous.origin.x,
                    self.previous.origin.z,
                ))
                .y;
        }
        self.next = None;
    }
    pub fn get_matrix(&self, screen_resolution: Vector2<u32>) -> Matrix4<f32> {
        let delta_position = self.previous.radius
            * Vector3::new(
                self.previous.phi.cos() * self.previous.theta.cos(),
                self.previous.theta.sin(),
                (self.previous.phi).sin() * self.previous.theta.cos(),
            );
        let face = Matrix4::look_at_rh(
            &Point::from(delta_position + self.previous.origin),
            &Point::from(self.previous.origin),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        let cam = Matrix4::new_perspective(
            screen_resolution.x as f32 / screen_resolution.y as f32,
            std::f32::consts::PI / 3.0,
            0.1,
            1000.0,
        );
        cam * face
    }
}
#[derive(Clone, Debug)]
struct Camera {
    /// Origin about which camera rotates
    origin: Vector3<f32>,
    /// how far away the camera is from its origin
    radius: f32,
    //angle
    phi: f32,
    theta: f32,
}
impl Camera {
    const FAR_CLIP: f32 = 1000.0;
}
