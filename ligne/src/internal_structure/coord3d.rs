use std::clone;

use num::bigint::ToBigInt;
use num::rational::Ratio;
use num::traits::float::FloatCore;
use num::{Integer, ToPrimitive};


pub type PhysicalCoord3D = Coord3D<f32>;
pub type VoxelCoord3D = Coord3D<Ratio<i32>>;

#[derive(Clone)]
pub struct Coord3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Coord3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

macro_rules! to_type {
    ($method_name: ident, $type:ty) => {
        pub fn $method_name(&self) -> Coord3D<Option<$type>> {
            Coord3D::new(
                self.x.$method_name(),
                self.y.$method_name(),
                self.z.$method_name(),
            )
        }
    };
}

impl<T> Coord3D<Ratio<T>> where T: Clone + Integer + ToPrimitive + ToBigInt {

    to_type!(to_f32, f32);
    to_type!(to_f64, f64);
    to_type!(to_i8, i8);
    to_type!(to_i16, i16);
    to_type!(to_i32, i32);
    to_type!(to_i64, i64);
    to_type!(to_i128, i128);
    to_type!(to_u8, u8);
    to_type!(to_u16, u16);
    to_type!(to_u32, u32);
    to_type!(to_u64, u64);
    to_type!(to_u128, u128);
    to_type!(to_usize, usize);
    to_type!(to_isize, isize);

}

impl<T> Coord3D<Ratio<T>> where T: Clone + Integer {
    
    pub fn floor(&self) -> Coord3D<Ratio<T>> {
        Coord3D::new(
            self.x.floor(),
            self.y.floor(),
            self.z.floor(),
        )
    }

    pub fn ceil(&self) -> Coord3D<Ratio<T>> {
        Coord3D::new(
            self.x.ceil(),
            self.y.ceil(),
            self.z.ceil(),
        )
    }

    pub fn round(&self) -> Coord3D<Ratio<T>> {
        Coord3D::new(
            self.x.round(),
            self.y.round(),
            self.z.round(),
        )
    }
}

impl VoxelCoord3D {
    pub fn to_physical(&self) -> PhysicalCoord3D {
        Coord3D::new(
            self.x.to_f32().unwrap(),
            self.y.to_f32().unwrap(),
            self.z.to_f32().unwrap(),
        )
    }
}

impl PhysicalCoord3D {
    pub fn to_voxel(&self) -> VoxelCoord3D {
        Coord3D::new(
            Ratio::approximate_float(self.x).unwrap(),
            Ratio::approximate_float(self.y).unwrap(),
            Ratio::approximate_float(self.z).unwrap(),
        )
    }
}