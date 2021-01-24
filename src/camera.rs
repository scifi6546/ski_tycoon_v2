use nalgebra::{Matrix4, Point, Vector2, Vector3};
pub struct Camera {
    /// Origin about which camera rotates
    origin: Vector3<f32>,
    /// how far away the camera is from its origin
    radius: f32,
    //angle
    phi: f32,
    theta: f32,
}
impl Camera {
    pub fn new(origin: Vector3<f32>, radius: f32, phi: f32, theta: f32) -> Self {
        Self {
            origin,
            radius,
            phi,
            theta,
        }
    }
    pub fn rotate_phi(&mut self, delta_phi: f32) {
        self.phi += delta_phi;
    }
    pub fn rotate_theta(&mut self, delta_theta: f32) {
        self.theta += delta_theta;
    }
    /// Increases by value proportional to delta radius
    pub fn update_radius(&mut self, delta_radius: f32) {
        self.radius += delta_radius * self.radius;
        if self.radius < 0.1 {
            self.radius = 0.1;
        }
    }
    pub fn translate(&mut self, translation: &Vector3<f32>) {
        self.origin += translation;
    }
    pub fn get_matrix(&self, screen_resolution: Vector2<u32>) -> Matrix4<f32> {
        let delta_position = self.radius
            * Vector3::new(
                self.phi.cos() * self.theta.cos(),
                self.theta.sin(),
                (self.phi).sin() * self.theta.cos(),
            );
        let face = Matrix4::look_at_rh(
            &Point::from(delta_position + self.origin),
            &Point::from(self.origin),
            &Vector3::new(0.0, 1.0, 0.0),
        );
        let cam = Matrix4::new_perspective(
            screen_resolution.x as f32 / screen_resolution.y as f32,
            3.14 / 3.0,
            0.1,
            1000.0,
        );
        let mat = cam * face;
        mat
    }
}
