use std::cell::{RefCell};
use std::rc::Rc;
use std::thread::yield_now;
use tui::style::Color;
use crate::TileType;

#[derive(Hash, Debug, Clone)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

impl Coords {
    pub fn from_coords(x: usize, y: usize) -> Self {
        Coords {
            x,
            y
        }
    }

    pub fn copy(&self) -> Self {
        Coords {
            x: self.x,
            y: self.y
        }
    }
}

impl PartialEq<Self> for Coords {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Coords {}

pub struct Node {
    pub x: usize,
    pub y: usize,
    pub distance: u64,
    pub ttype: TileType,
    pub neighbors: Vec<Coords>,
    pub visited: bool,
}

impl Node {
    pub fn from_coords(x: usize, y: usize, ttype: TileType) -> Self {
        Node {
            x,
            y,
            distance: 0,
            ttype,
            neighbors: Vec::with_capacity(4),
            visited: false,
        }
    }

    pub fn shallow_clone(&self) -> Self {
        Node {
            x: self.x,
            y: self.y,
            distance: 0,
            ttype: self.ttype.clone(),
            neighbors: Vec::new(),
            visited: false
        }
    }

    pub fn add_neighbor(&mut self, coords: Coords) {
        self.neighbors.push(coords);
    }

    pub fn coords(&self) -> Coords {
        Coords {
            x: self.x,
            y: self.y
        }
    }
}


pub trait CoordAccess<T> {
    fn get(&self, coord: &Coords) -> &T;
    fn get_mut(&mut self, coord: &Coords) -> &mut T;
}

impl <T>CoordAccess<T> for Vec<Vec<T>> {
    fn get(&self, coord: &Coords) -> &T {
        return &self[coord.y][coord.x];
    }
    fn get_mut(&mut self, coord: &Coords) -> &mut T {
        return &mut self[coord.y][coord.x];
    }
}

pub trait TileTypeInformation {
    fn is_traversable(&self) -> bool;
    fn allow_building(&self) -> bool;
    fn color(&self) -> Color;
}

impl TileTypeInformation for Node {
    fn is_traversable(&self) -> bool {
        match self.ttype {
            TileType::Free => true,
            TileType::Unbuildable => true,
            TileType::Void => false,
            TileType::Spawn => true,
            TileType::Exit => true,
            TileType::Occupied => false,
            TileType::Path => true,
        }
    }

    fn allow_building(&self) -> bool {
        match self.ttype {
            TileType::Free => true,
            TileType::Unbuildable => false,
            TileType::Void => false,
            TileType::Spawn => false,
            TileType::Exit => false,
            TileType::Occupied => false,
            TileType::Path => true,
        }
    }

    fn color(&self) -> Color {
        match self.ttype {
            TileType::Free => Color::Rgb(0xa1, 0x6b, 0x55),
            TileType::Unbuildable => Color::Rgb(0x6e, 0x3b, 0x27),
            TileType::Void => Color::Reset,
            TileType::Spawn => Color::Red,
            TileType::Exit => Color::Green,
            TileType::Occupied => Color::Rgb(0x6e, 0x1f, 0xa6),
            TileType::Path => Color::Rgb(0xf7, 0xed, 0x23),
        }
    }
}

static NEIGHBOR_4: &'static [[i32; 2]] = &[[0, 1], [-1, 0], [1, 0], [0, -1]];

pub fn fill_neighbours(map: &Vec<Vec<Rc<RefCell<Node>>>>) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let max_y = map.len();


    for row in map {
        let max_x = row.len();
        for node in row {
            if !node.borrow().is_traversable() {
                x += 1;
                continue;
            }

            for coord in NEIGHBOR_4 {
                if x == 0 && coord[0] < 0 ||
                    y == 0 && coord[1] < 0 {
                    continue;
                }

                let neighbor_x = usize::try_from(x as i32 + coord[0]).ok().unwrap_or(0);
                let neighbor_y = usize::try_from(y as i32 + coord[1]).ok().unwrap_or(0);

                if neighbor_x >= max_x || neighbor_y >= max_y  {
                    continue;
                }

                // Just skip adding non traversable neighbors
                if !&map[neighbor_y][neighbor_x].borrow().is_traversable() {
                    continue;
                }

                node.borrow_mut().add_neighbor(Coords::from_coords(neighbor_x, neighbor_y));
            }
            x += 1;
        }

        y += 1;
        x = 0;
    }
}

pub fn reset_nodes(map: &Vec<Vec<Rc<RefCell<Node>>>>) {
    for row in map {
        for node in row {
            let mut node_mut = node.borrow_mut();
            node_mut.distance = 0;
            node_mut.visited = false;
        }
    }
}

pub fn find_start_node(map: &Vec<Vec<Rc<RefCell<Node>>>>) -> Option<&Rc<RefCell<Node>>> {
    for row in map {
        for node in row {
            if node.borrow().ttype == TileType::Spawn {
                return Some(node);
            }
        }
    }

    None
}

pub fn find_nodes_of_type(map: &Vec<Vec<Rc<RefCell<Node>>>>, ttype: TileType) -> Vec<Coords> {
    let mut coords = Vec::new();
    for row in map {
        for node in row {
            if node.borrow().ttype == ttype {
                coords.push(node.borrow().coords());
            }
        }
    }

    return coords;
}
