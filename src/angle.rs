#[repr(C)]
pub struct Deg(pub f32);

impl Deg {
    fn to_rad(&self) -> Rad {
        Rad(self.0 * std::f32::consts::PI / 180.0)
    }
}

#[repr(C)]
pub struct Rad(pub f32);

impl Rad {
    fn to_deg(&self) -> Deg {
        Deg(self.0 * 180.0 / std::f32::consts::PI)
    }
}