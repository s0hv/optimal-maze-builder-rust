use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use crate::graphs::utils::{Coords, reset_nodes};
use crate::{Node, TileType, TileTypeInformation};

pub fn collect_dfs_path(node: &Rc<RefCell<Node>>, map: &Vec<Vec<Rc<RefCell<Node>>>>, path: &mut HashSet<Coords>) {
    for coord in &node.borrow().neighbors {
        let neighbor = map[coord.y][coord.x].borrow();
        if neighbor.visited && neighbor.distance == node.borrow().distance - 1  && !path.contains(&neighbor.coords()) {
            if neighbor.distance == 0 {
                continue;
            }

            path.insert(neighbor.coords());

            collect_dfs_path(&map[coord.y][coord.x], &map, path);
        }
    }
}

pub fn dfs(start_node: &Rc<RefCell<Node>>, map: &Vec<Vec<Rc<RefCell<Node>>>>, node_count: usize) -> Option<(u64, HashSet<Coords>)> {
    reset_nodes(map);
    static END_TYPE: TileType = TileType::Exit;
    let mut deque: VecDeque<&Rc<RefCell<Node>>> = VecDeque::with_capacity(node_count);
    start_node.borrow_mut().visited = true;
    deque.push_back(start_node);

    while !deque.is_empty() {
        let node_ref = deque.pop_front().unwrap();
        let node = node_ref.borrow();

        if node.ttype == END_TYPE {
            let distance = node.distance;
            let mut path: HashSet<Coords> = HashSet::new();
            collect_dfs_path(node_ref, &map, &mut path);
            return Some((distance, path))
        }

        for coord in &node.neighbors {
            let neighbor = &map[coord.y][coord.x];
            if !neighbor.borrow().visited && neighbor.borrow().is_traversable() {
                neighbor.borrow_mut().visited = true;
                neighbor.borrow_mut().distance = node.distance + 1;

                deque.push_back(neighbor);
            }
        }
    }

    None
}
