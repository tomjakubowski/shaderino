use glium::{uniform, uniforms::Uniforms};
use std::time::Instant;

pub struct GlslViewer {
    epoch: Instant,
    mouse: [f32; 2],
    resolution: [f32; 2],
    time: Instant,
}

impl GlslViewer {
    pub fn new() -> GlslViewer {
        let now = Instant::now();
        GlslViewer {
            epoch: now,
            mouse: [0.0, 0.0],
            resolution: [0.0, 0.0],
            time: now,
        }
    }

    pub fn time(&self) -> Instant {
        self.time
    }

    pub fn update(&mut self) {
        self.time = Instant::now();
    }

    pub fn set_mouse(&mut self, x: f32, y: f32) {
        self.mouse = [x, y]
    }

    pub fn set_resolution(&mut self, wid: f32, hei: f32) {
        self.resolution = [wid, hei]
    }

    pub fn resolution(&self) -> [f32; 2] {
        self.resolution
    }

    pub fn uniforms(&self) -> impl Uniforms {
        let since_epoch = self.time - self.epoch;
        let u_time = since_epoch.as_secs() as f64 + (since_epoch.subsec_micros() as f64 * 1e-6);
        let u_time = u_time as f32;
        uniform! {
            u_mouse: self.mouse,
            u_resolution: self.resolution,
            u_time: u_time
        }
    }
}
