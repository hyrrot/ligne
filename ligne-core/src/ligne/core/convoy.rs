// use crate::ligne::core::object::Object;
// use crate::ligne::core::types::Coord3D;

// use super::types::Offset3D;

// pub trait Convoy {
//     fn get_object(&self, index: u8) -> &dyn Object;
//     fn set_object(&mut self, index: u8, object: &dyn Object);
//     fn get_position(&self, index: u8) -> Coord3D;
//     fn set_position(&mut self, index: u8, position: Coord3D);
//     fn get_offset(&self, index: u8) -> Offset3D;
//     fn set_offset(&mut self, index: u8, offset: Offset3D);
//     fn get_object_relative_position(&self, index: u8) -> Coord3D;
//     fn set_object_relative_position(&mut self, index: u8, position: Coord3D);

//     fn draw(&self);
// }

// pub struct RailwayConvoy {}

// impl Convoy for RailwayConvoy {
//     fn draw(&self) {
//         println!("Draw railway convoy");
//     }

//     fn get_object(&self, index: u8) -> &dyn Object {
//         println!("Get object from railway convoy");
//         &crate::ligne::core::object::ObjectImpl {}
//     }

//     fn set_object(&mut self, index: u8, object: &dyn Object) {
//         println!("Set object to railway convoy");
//     }

//     fn get_object_position(&self, index: u8) -> Coord3D {
//         println!("Get object position from railway convoy");
//         Coord3D { x: 0, y: 0, z: 0 }
//     }

// }
