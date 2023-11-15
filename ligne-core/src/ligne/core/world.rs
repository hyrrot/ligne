use crate::ligne::core::types::Coord3D;
use crate::ligne::core::voxel::EmptyVoxel;
use crate::ligne::core::voxel::Voxel;

// pub struct World {

//     width: i32,
//     height: i32,
//     depth: i32,
//     voxels: Vec<Voxel>,

// }

pub trait World {
    fn new(width: u32, height: u32, depth: u32) -> Box<Self>;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn depth(&self) -> u32;

    fn get_voxel(&self, coord: Coord3D) -> Option<&Box<dyn Voxel>>;
    fn set_voxel(&mut self, coord: Coord3D, voxel: Box<dyn Voxel>);
}

/// ```rust
/// use ligne_core::ligne::core::types::Coord3D;
/// use ligne_core::ligne::core::world::WorldImpl;
/// use crate::ligne_core::ligne::core::world::World;
/// use ligne_core::ligne::core::voxel::VoxelType;
/// let world = WorldImpl::new(10, 10, 10);
/// assert_eq!(world.width(), 10);
/// assert_eq!(world.height(), 10);
/// assert_eq!(world.depth(), 10);
/// assert!( world.get_voxel(Coord3D { x: 0, y: 0, z: 0 }).is_some_and(|v| matches!(v.get_type(), VoxelType::Empty) ) );
/// ```
pub struct WorldImpl {
    _width: u32,
    _height: u32,
    _depth: u32,
    _voxels: Vec<Box<dyn Voxel>>,
}
impl WorldImpl {
    fn index(&self, coord: Coord3D) -> usize {
        (coord.x + coord.y * self.width() + coord.z * self.width() * self.height()) as usize
    }
}

impl World for WorldImpl {
    fn width(&self) -> u32 {
        self._width
    }

    fn height(&self) -> u32 {
        self._height
    }

    fn depth(&self) -> u32 {
        self._depth
    }

    fn new(width: u32, height: u32, depth: u32) -> Box<Self> {
        let mut voxels = Vec::<Box<dyn Voxel>>::with_capacity((width * height * depth) as usize);
        for _ in 0..(width * height * depth) {
            // TODO: apply abstract factory pattern
            voxels.push(Box::new(EmptyVoxel {}));
        }
        Box::new(Self {
            _voxels: voxels,
            _width: width,
            _height: height,
            _depth: depth,
        })
    }

    fn get_voxel(&self, coord: Coord3D) -> Option<&Box<dyn Voxel>> {
        self._voxels.get(self.index(coord))
    }

    fn set_voxel(&mut self, coord: Coord3D, voxel: Box<dyn Voxel>) {
        let idx = self.index(coord);
        self._voxels[idx] = voxel;
    }
}
