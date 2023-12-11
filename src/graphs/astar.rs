use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use pathfinding::directed::astar::{astar_bag, astar as astar_impl};
use pathfinding::num_traits::abs_sub;
use crate::graphs::utils::Coords;
use crate::{Node, TileType, TileTypeInformation};


pub fn astar(start_node: &Rc<RefCell<Node>>, map: &Vec<Vec<Rc<RefCell<Node>>>>, goals: &Vec<Coords>) -> Option<(u64, HashSet<Coords>)> {
    let res = astar_impl(
        &start_node.borrow().coords(),
        |c| map[c.y][c.x]
            .borrow().neighbors.iter()
            .filter(|c| map[c.y][c.x].borrow().is_traversable())
            .map(|c| (c.clone(), 1)).collect::<Vec<(Coords, i32)>>(),
        |c| goals.iter().map(|cc| abs_sub(cc.x as i32, c.x as i32) + abs_sub(cc.y as i32, c.y as i32)).min().unwrap(),
        |c| map[c.y][c.x].borrow().ttype == TileType::Exit
    );

    if res.is_none() {
        return None;
    }

    let paths = res.unwrap();
    //let result: HashSet<Coords> = HashSet::from_iter(paths.0.into_iter().flatten().into_iter());
    return Some((paths.1 as u64, HashSet::from_iter(paths.0)));
}
