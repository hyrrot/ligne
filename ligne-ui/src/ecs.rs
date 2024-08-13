pub mod ecs;
use num::rational::*;

#[derive(Component)]
pub enum VehicleType {
    Train,
    Car,
    Truck,
    Bus,
    Bike,
    Scooter,
    Motorcycle,
    Boat,
    Ship,
    Plane,
    Helicopter,
    Drone,
    Rocket,
    Satellite,
    Spacecraft,
    UFO,
    Other,
}

#[derive(Component)]
pub struct PositionInConvoy(u16);

#[derive(Component)]
pub struct Vehicles(Vec<Entity>);

#[derive(Component)]
pub struct Voxels(Vec<Entity>);

#[derive(Component)]
pub struct MapCoord {
    x: Ratio<i16>,
    y: Ratio<i16>,
    z: Ratio<i16>,
}

#[derive(Component)]
pub enum VoxelType {
    Empty,
    Rail,
    Vehicle,
}

#[derive(Component)]
pub struct VehicleTexture(i16);

#[derive(Component)]
pub struct VehiclelElementID(u8);

#[derive(Component)]
pub struct Vehicle {}

#[derive(Component)]
pub struct Convoy {}

#[derive(Component)]
pub struct Grid {}

#[derive(Bundle)]
pub struct VehicleBundle {
    marker: Vehicle,
    name: string,
    vehicle_type: VehicleType,
    position_in_convoy: PositionInConvoy,
    relative_map_coord: MapCoord,
    sprite_bundle: SpriteBundle,
}

#[derive(Bundle)]
pub struct ConvoyBundle {
    marker: Convoy,
    vehicles: Vehicles,
    relative_map_coord: MapCoord,
}

#[derive(Bundle)]
pub struct GridBundle {
    marker: Grid,
    voxels: Voxels,
}

#[derive(Bundle)]
pub struct VoxelBundle {
    marker: Voxel,
    relative_map_coord: MapCoord,
    sprite_bundle: SpriteBundle,
    voxel_type: VoxelType,
}
