use std::cell::{RefCell};
use std::collections::{HashMap};
use std::collections::HashSet;
use std::rc::Rc;
use std::time;
use crate::{TileType, TileTypeInformation, CoordAccess, find_start_node};
use crate::builders::common::BuilderResult;
use crate::graphs::dfs::{dfs};
use crate::graphs::astar::{astar};
use crate::graphs::utils::{Coords, find_nodes_of_type, Node};

struct CutoffMeta {
    pub max_towers: u64,
    pub combinations: u64,
    pub node_count: usize,
    pub best_towers: Vec<Vec<Coords>>,
    pub best_dist: Option<u64>,
    pub processed_coords: HashMap<u64, HashSet<Coords>>,
    pub goals: Vec<Coords>
}

fn cutoff_recursive<'a>(start_node: &Rc<RefCell<Node>>, map: &Vec<Vec<Rc<RefCell<Node>>>>, meta: &RefCell<CutoffMeta>, placed_towers: &Vec<Coords>, towers_left: u64) {
    meta.borrow_mut().combinations += 1;
    let result = astar(start_node, map, &meta.borrow().goals);

    // unsolvable
    if result.is_none() {
        if towers_left > 0 && placed_towers.len() > 0 {
            meta.borrow_mut().processed_coords.get_mut(&towers_left).unwrap().insert(placed_towers.last().unwrap().clone());
        }

        return;
    }

    let (dist, coords_found) = result.unwrap();

    if meta.borrow().best_dist.is_none() || dist > meta.borrow().best_dist.unwrap() {
        meta.borrow_mut().best_dist = Some(dist);
        meta.borrow_mut().best_towers = vec![placed_towers.iter().map(|c| { c.copy() }).collect()];
    } else if meta.borrow().best_dist.unwrap() == dist {
        meta.borrow_mut().best_towers.push(placed_towers.iter().map(|c| { c.copy() }).collect());
    }

    if towers_left == 0 {
        return;
    }

    let mut coords: HashSet<Coords> = coords_found;

    for count in towers_left..meta.borrow().max_towers + 1 {
        coords = &coords - &meta.borrow().processed_coords[&count]
    }

    for coord in &coords {
        let node = map.get(coord);

        if !node.borrow().allow_building() {
            continue;
        }

        node.borrow_mut().ttype = TileType::Occupied;

        let mut new_placed: Vec<Coords> = placed_towers.iter().map(|c| { c.copy() }).collect();
        new_placed.push(coord.copy());

        cutoff_recursive(start_node, map, meta, &new_placed, towers_left - 1);

        node.borrow_mut().ttype = TileType::Free;
        meta.borrow_mut().processed_coords.get_mut(&towers_left).unwrap().insert(coord.copy());
        if towers_left > 1 {
            meta.borrow_mut().processed_coords.get_mut(&(towers_left - 1)).unwrap().clear();
        }
    }
}

pub fn cutoff_builder(start_node: &Rc<RefCell<Node>>, map: &Vec<Vec<Rc<RefCell<Node>>>>, node_count: usize, max_towers: u64) -> BuilderResult {
    let meta = RefCell::new(CutoffMeta {
        max_towers,
        combinations: 0,
        node_count,
        best_towers: vec![],
        best_dist: None,
        processed_coords: HashMap::new(),
        goals: find_nodes_of_type(&map, TileType::Exit)
    });

    for i in 1..max_towers + 1 {
        meta.borrow_mut().processed_coords.insert(i, HashSet::new());
    }

    let now = time::Instant::now();

    cutoff_recursive(start_node, map, &meta, &Vec::new(), max_towers);

    let took = time::Instant::now() - now;

    println!("N: {} t: {:3}ms", meta.borrow().combinations, took.as_millis());

    return BuilderResult {
        duration: took,
        best_towers: std::mem::take(&mut meta.borrow_mut().best_towers)
    };
}
