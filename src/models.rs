use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum TileType {
    Free = 0,
    Unbuildable = 1,
    Void = 2,
    Spawn = 3,
    Exit = 4,
    Occupied = 5,
    Path = 6
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapInfo {
    pub map: Vec<Vec<TileType>>
}
