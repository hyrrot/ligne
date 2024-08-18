use num::rational::Ratio;
use num::ToPrimitive;

pub struct Coord3D<T: Clone> {
    x: T,
    y: T,
    z: T,
}

pub type OffsetedCoord3D = Coord3D<OffsetedCoordValue>;
pub type OffsetedCoordValue = Ratio<i32>;

impl OffsetedCoord3D {

    pub fn new(x: OffsetedCoordValue, y: OffsetedCoordValue, z: OffsetedCoordValue) -> OffsetedCoord3D {
        OffsetedCoord3D {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn from_integer(x: i32, y: i32, z: i32) -> OffsetedCoord3D {
        OffsetedCoord3D {
            x: Ratio::from_integer(x),
            y: Ratio::from_integer(y),
            z: Ratio::from_integer(z),
        }
    }

    pub fn floor(&self) -> Coord3D<i32> {
        Coord3D {
            x: self.x.floor().to_integer(),
            y: self.y.floor().to_integer(),
            z: self.z.floor().to_integer(),
        }
    }

    pub fn round(&self) -> Coord3D<i32> {
        Coord3D {
            x: self.x.round().to_integer(),
            y: self.y.round().to_integer(),
            z: self.z.round().to_integer(),
        }
    }

    pub fn ceil(&self) -> Coord3D<i32> {
        Coord3D {
            x: self.x.ceil().to_integer(),
            y: self.y.ceil().to_integer(),
            z: self.z.ceil().to_integer(),
        }
    }

    pub fn to_f32(&self) -> Coord3D<f32> {
        Coord3D {
            x: self.x.to_f32().unwrap(),
            y: self.y.to_f32().unwrap(),
            z: self.z.to_f32().unwrap(),
        }
    }

    pub fn mut_shift(&mut self, x: OffsetedCoordValue, y: OffsetedCoordValue, z: OffsetedCoordValue) {
        self.x = self.x + x;
        self.y = self.y + y;
        self.z = self.z + z;
    }

    pub fn mut_update(&mut self, x: OffsetedCoordValue, y: OffsetedCoordValue, z: OffsetedCoordValue) {
        self.x = x;
        self.y = y;
        self.z = z;
    }




}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_new() {
        let coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        assert_eq!(coord.x, Ratio::new(5, 2));
        assert_eq!(coord.y, Ratio::new(3, 2));
        assert_eq!(coord.z, Ratio::new(7, 2));
    }

    #[test]
    fn test_from_integer() {
        let coord = OffsetedCoord3D::from_integer(5, 3, 7);
        assert_eq!(coord.x, Ratio::new(5, 1));
        assert_eq!(coord.y, Ratio::new(3, 1));
        assert_eq!(coord.z, Ratio::new(7, 1));
    }

    #[test]
    fn test_floor() {
        let coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        let floor = coord.floor();
        assert_eq!(floor.x, 2);
        assert_eq!(floor.y, 1);
        assert_eq!(floor.z, 3);
    }

    #[test]
    fn test_round() {
        let coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        let round = coord.round();
        assert_eq!(round.x, 3);
        assert_eq!(round.y, 2);
        assert_eq!(round.z, 4);
    }

    #[test]
    fn test_ceil() {
        let coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        let ceil = coord.ceil();
        assert_eq!(ceil.x, 3);
        assert_eq!(ceil.y, 2);
        assert_eq!(ceil.z, 4);
    }

    #[test]
    fn test_to_f32() {
        let coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        let f32 = coord.to_f32();
        assert_eq!(f32.x, 2.5);
        assert_eq!(f32.y, 1.5);
        assert_eq!(f32.z, 3.5);
    }

    #[test]
    fn test_mut_shift() {
        let mut coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        coord.mut_shift(Ratio::new(1, 2), Ratio::new(1, 2), Ratio::new(1, 2));
        assert_eq!(coord.x, Ratio::new(3, 1));
        assert_eq!(coord.y, Ratio::new(2, 1));
        assert_eq!(coord.z, Ratio::new(4, 1));
    }

    #[test]
    fn test_mut_update() {
        let mut coord = OffsetedCoord3D::new(Ratio::new(5, 2), Ratio::new(3, 2), Ratio::new(7, 2));
        coord.mut_update(Ratio::new(1, 2), Ratio::new(1, 2), Ratio::new(1, 2));
        assert_eq!(coord.x, Ratio::new(1, 2));
        assert_eq!(coord.y, Ratio::new(1, 2));
        assert_eq!(coord.z, Ratio::new(1, 2));
    }

}