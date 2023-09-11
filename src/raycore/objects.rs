use raylib_ffi::{
    colors::{GREEN, RED},
    enums::CameraProjection,
    enums::MouseButton,
    BoundingBox, Camera, Color, DrawCubeV, DrawCubeWiresV, GetMousePosition, GetMouseRay,
    GetRayCollisionBox, IsMouseButtonPressed, Ray, RayCollision, Vector3,
};

pub enum CameraMode {
    CameraFree,
}

pub trait Object {
    fn is_hit_by_ray(&self) -> bool;
    fn get_bbox(&self) -> BoundingBox;
    fn set_hit_by_ray(&mut self, val: bool);
    fn drawobj(&self);
}
pub struct Cube {
    pub is_hit_by_ray: bool,
    pub position: Vector3,
    pub size: Vector3,
    pub color: Color,
    pub outline: Option<Color>,
    pub bbox: BoundingBox,
}
pub struct SceneCam {
    pub camera: Box<Camera>,
    pub selector: CollisionRay,
}

pub struct CollisionRay {
    pub ray: Ray,
    pub ray_collison: RayCollision,
}

impl CollisionRay {
    fn new() -> Self {
        Self {
            ray: Ray {
                position: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                direction: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            ray_collison: RayCollision {
                hit: false,
                distance: 0.0,
                point: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        }
    }
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
            selector: CollisionRay::new(),
        }
    }
    pub fn handle_collision(&mut self, objects: &mut Vec<Box<dyn Object>>) {
        unsafe {
            if IsMouseButtonPressed(MouseButton::Left as i32) {
                for obj in objects {
                    if !self.selector.ray_collison.hit {
                        self.selector.ray = GetMouseRay(GetMousePosition(), *self.camera as Camera);
                        self.selector.ray_collison =
                            GetRayCollisionBox(self.selector.ray, obj.get_bbox());
                        obj.set_hit_by_ray(if self.selector.ray_collison.hit {
                            true
                        } else {
                            false
                        });
                        obj.set_hit_by_ray(true);
                    } else {
                        obj.set_hit_by_ray(false);
                        self.selector.ray_collison.hit = false
                    }
                }
            }
        }
    }
}

impl Object for Cube {
    fn drawobj(&self) {
        unsafe {
            DrawCubeV(self.position, self.size, self.color);
            if self.is_hit_by_ray {
                if let Some(color) = self.outline {
                    DrawCubeWiresV(self.position, self.size, color);
                }
                DrawCubeWiresV(
                    self.position,
                    Vector3 {
                        x: self.size.x + 0.2,
                        y: self.size.y + 0.2,
                        z: self.size.z + 0.2,
                    },
                    GREEN,
                )
            } else {
                if let Some(color) = self.outline {
                    DrawCubeWiresV(self.position, self.size, color);
                }
            }
        };
    }

    fn is_hit_by_ray(&self) -> bool {
        self.is_hit_by_ray
    }

    fn set_hit_by_ray(&mut self, val: bool) {
        self.is_hit_by_ray = val;
    }

    fn get_bbox(&self) -> BoundingBox {
        self.bbox
    }
}
impl Cube {
    pub fn new(position: Vector3, size: Vector3, color: Color, outline: Option<Color>) -> Self {
        Self {
            position,
            size,
            color,
            outline,
            is_hit_by_ray: false,
            bbox: BoundingBox {
                min: {
                    Vector3 {
                        x: position.x - size.x / 2.0,
                        y: position.y - size.y / 2.0,
                        z: position.z - size.z / 2.0,
                    }
                },
                max: Vector3 {
                    x: position.x + size.x / 2.0,
                    y: position.y + size.y / 2.0,
                    z: position.z + size.z / 2.0,
                },
            },
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::new(
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
        )
    }
}
