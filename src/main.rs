mod raycore;
use raycore::prelude::*;
use raylib_ffi::WindowShouldClose;

fn main() {
    Window::new().name("Hello").size((1920, 1080)).build();
    let mut scene = Scene::new(60);
    scene.add_object(Box::new(Cube::default()));
    unsafe {
        while !WindowShouldClose() {
            scene.render();
        }
    }
}
