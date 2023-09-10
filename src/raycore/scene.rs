use raylib_ffi::{
    colors::{GREEN, RAYWHITE, RED},
    BeginDrawing, BeginMode3D, Camera3D, ClearBackground, DrawGrid, EndMode3D, UpdateCamera,
    Vector3,
};

use super::{
    objects::{CameraMode, Object, SceneCam},
    prelude::Cube,
};

pub enum ObjectType {
    CUBE,
}
pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub maincam: SceneCam,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            maincam: SceneCam::new(CameraMode::CameraFree),
        }
    }
    pub fn add_object(&mut self, objtype: ObjectType) {
        match objtype {
            ObjectType::CUBE => self.objects.push(Box::new(Cube::new(
                Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                },
                RED,
                Some(GREEN),
            ))),
        }
    }
    pub fn render(&mut self) {
        unsafe {
            raylib_ffi::BeginDrawing();
            {
                UpdateCamera(
                    self.maincam.camera.as_mut() as *mut Camera3D,
                    raylib_ffi::enums::CameraMode::Free as i32,
                );
                ClearBackground(RAYWHITE);
                BeginMode3D(*self.maincam.camera.as_ref());
                DrawGrid(255, 1.0);
                for obj in &self.objects {
                    obj.drawobj()
                }

                EndMode3D();
            }
            raylib_ffi::EndDrawing();
        }
    }
}
