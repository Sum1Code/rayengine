use raylib_ffi::{enums::CameraProjection, Camera, Color, DrawCubeV, DrawCubeWiresV, Vector3};

pub enum CameraMode {
    CameraFree,
}

pub trait Object {
    fn drawobj(&self);
}
pub struct Cube {
    pub position: Vector3,
    pub size: Vector3,
    pub color: Color,
    pub outline: Option<Color>,
}
pub struct SceneCam {
    pub camera: Box<Camera>,
}

impl SceneCam {
    pub fn new(mode: CameraMode) -> Self {
        let cam = match mode {
            CameraMode::CameraFree => {
                let cam = Camera {
                    position: Vector3 {
                        x: 10.0,
                        y: 10.0,
                        z: 10.0,
                    },
                    up: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    target: Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    projection: CameraProjection::Perspective as i32,
                    fovy: 45.0,
                };
                cam
            }
        };
        Self {
            camera: Box::new(cam),
        }
    }
}

impl Object for Cube {
    fn drawobj(&self) {
        unsafe {
            DrawCubeV(self.position, self.size, self.color);
            if let Some(color) = self.outline {
                DrawCubeWiresV(self.position, self.size, color)
            }
        };
    }
}
impl Cube {
    pub fn new(position: Vector3, size: Vector3, color: Color, outline: Option<Color>) -> Self {
        Self {
            position,
            size,
            color,
            outline,
        }
    }
}
