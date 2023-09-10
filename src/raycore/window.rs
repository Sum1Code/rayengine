use raylib_ffi::{colors::RAYWHITE, rl_str, Color, InitWindow};

pub struct Window {
    window: BaseWindow,
}

struct BaseWindow {
    name: String,
    size: (i32, i32),
    refresh_color: Color,
}

impl Window {
    pub fn new() -> Self {
        Window {
            window: BaseWindow::default(),
        }
    }
    pub fn name(mut self, name: &str) -> Self {
        self.window.name = name.to_owned();
        self
    }
    pub fn size(mut self, size: (i32, i32)) -> Self {
        self.window.size = size;
        self
    }
    pub fn build(self) {
        unsafe {
            InitWindow(
                self.window.size.0,
                self.window.size.1,
                rl_str!(self.window.name),
            )
        };
    }
}
impl Default for BaseWindow {
    fn default() -> Self {
        Self {
            name: "Hello".to_owned(),
            size: (0, 0),
            refresh_color: RAYWHITE,
        }
    }
}
