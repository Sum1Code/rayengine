use raylib_ffi::{
    colors::{MAROON, RAYWHITE},
    enums::KeyboardKey,
    BeginMode3D, Camera3D, ClearBackground, DisableCursor, DrawGrid, DrawRay, EnableCursor,
    EndMode3D, IsCursorHidden, IsKeyDown, IsKeyPressed, SetTargetFPS, UpdateCamera,
};

use super::objects::{CameraMode, Object, SceneCam};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub maincam: SceneCam,
    pub target_fps: i32,
}

impl Scene {
    pub fn new(target_fps: i32) -> Self {
        Self {
            objects: Vec::new(),
            maincam: SceneCam::new(CameraMode::CameraFree),
            target_fps,
        }
    }
    pub fn add_object(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }
    pub fn render(&mut self) {
        unsafe {
            SetTargetFPS(self.target_fps);
            raylib_ffi::BeginDrawing();
            {
                if IsCursorHidden() {
                    UpdateCamera(
                        self.maincam.camera.as_mut() as *mut Camera3D,
                        raylib_ffi::enums::CameraMode::Free as i32,
                    );
                }
                self.maincam.handle_collision(&mut self.objects);
                ClearBackground(RAYWHITE);
                self.handle_inputs();
                BeginMode3D(*self.maincam.camera.as_ref());
                DrawGrid(255, 1.0);
                for obj in &self.objects {
                    obj.drawobj()
                }
                DrawRay(self.maincam.selector.ray, MAROON);
                EndMode3D();
            }
            raylib_ffi::EndDrawing();
        }
    }
    pub fn handle_inputs(&mut self) {
        unsafe {
            if IsKeyDown(KeyboardKey::LeftShift as i32) {
                self.target_fps = 200;
            } else {
                self.target_fps = 60;
            }
            if IsKeyPressed(KeyboardKey::LeftAlt as i32) {
                if IsCursorHidden() {
                    EnableCursor();
                } else {
                    DisableCursor();
                }
            }
        }
    }
}
