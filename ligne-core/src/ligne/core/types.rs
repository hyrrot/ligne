use num_rational::*;

pub struct Coord3D {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct Offset3D {
    pub x: Ratio<i8>,
    pub y: Ratio<i8>,
    pub z: Ratio<i8>,
}
