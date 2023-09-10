mod raycore;
use raycore::prelude::*;
use raylib_ffi::{SetTargetFPS, WindowShouldClose};

fn main() {
    Window::new().name("Hello").size((1920, 1080)).build();
    let mut scene = Scene::new();
    scene.add_object(ObjectType::CUBE);
    unsafe {
        SetTargetFPS(60);
        while !WindowShouldClose() {
            scene.render();
        }
    }
}
