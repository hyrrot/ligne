use std::fmt;

pub enum VoxelType {
    Empty,
    Terrain,
    Rail,
    Others,
}

pub enum RailType {
    StraightOrCurve(StraightOrCurveRailType),
    Branch(BranchRailType),
}

#[allow(non_camel_case_types)]
pub enum StraightOrCurveRailType {
    NW_E = 0,
    NW_SE = 1,
    NW_S = 2,
    N_SE = 3,
    N_S = 4,
    N_SW = 5,
    NE_S = 6,
    NE_SW = 7,
    NE_W = 8,
    E_SW = 9,
    E_W = 10,
    SE_W = 11,
}

#[allow(non_camel_case_types)]
pub enum BranchRailType {
    NW_SE_E = 16,
    NW_SE_S = 17,
    N_S_SE = 18,
    N_S_SW = 19,
    NE_SW_S = 20,
    NE_SW_W = 21,
    E_W_SW = 22,
    E_W_NW = 23,
    SE_NW_W = 24,
    SE_NW_N = 25,
    N_S_NW = 26,
    N_S_NE = 27,
    NE_SW_N = 28,
    NE_SW_E = 29,
    E_W_NE = 30,
    E_W_SE = 31,
}

pub trait Voxel {
    // Get VoxelType
    fn get_type(&self) -> VoxelType;

    // A function that draws the voxel.
    fn draw(&self);
}

impl fmt::Debug for dyn Voxel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Voxel")
    }
}

pub struct VoxelImpl {}

impl Voxel for VoxelImpl {
    fn get_type(&self) -> VoxelType {
        VoxelType::Others
    }

    fn draw(&self) {
        println!("Draw voxel");
    }
}

pub struct EmptyVoxel {}

impl Voxel for EmptyVoxel {
    fn get_type(&self) -> VoxelType {
        VoxelType::Empty
    }

    fn draw(&self) {
        println!("Draw empty voxel");
    }
}
pub struct TerrainVoxel {}

impl Voxel for TerrainVoxel {
    fn get_type(&self) -> VoxelType {
        VoxelType::Terrain
    }

    fn draw(&self) {
        println!("Draw terrain voxel");
    }
}

pub struct RailVoxel {
    rail_type: RailType,
}

impl Voxel for RailVoxel {
    fn get_type(&self) -> VoxelType {
        VoxelType::Rail
    }

    fn draw(&self) {
        println!("Draw rail voxel");
    }
}
