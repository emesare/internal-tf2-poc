#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix([[f32; 4]; 3]);

impl Matrix {
    pub fn empty() -> Self {
        Self(Default::default())
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    // this code below was written just for the example outlined in the hooks/panel.rs, normally you would want to implement a `Into` trait that does the conversions.
    pub fn x(&self) -> i32 {
        self.x as i32
    }

    pub fn y(&self) -> i32 {
        self.y as i32
    }

    pub fn z(&self) -> i32 {
        self.z as i32
    }

    pub fn convert(&self) -> (i32, i32, i32) {
        (self.x(), self.y(), self.z())
    }
}
