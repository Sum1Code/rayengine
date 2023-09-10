# RAYENGINE << STILL IN DEVELOPMENT >>
New Rayengine built in rust instead.. still the same inside though
Rayengine is targeted to be just another 3D game engine.. using raylib is cheating though..

## HOW TO BUILD 
- `git clone https://github.com/Sum1Code/rayengine.git`
- `cd rayengine`
- `cargo build`

## EXAMPLE
```rs
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
```

## Screenshot
![Rayengine](./assets/rayengine-p1.png)

## GOALS:
- [ ] stable 3D environment
- [ ] good UI
- [ ] optional modules [libvideo, lib2d, ...] 
