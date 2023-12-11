use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, fs};
use std::cell::{RefCell};
use std::io::ErrorKind;
use std::rc::Rc;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Style},
    widgets::{Block},
    Frame, Terminal,
};
use tui::layout::Rect;
use serde_json::Result as JsonResult;
use crate::builders::cutoff::{cutoff_builder};
use crate::graphs::utils::{fill_neighbours, find_start_node, Node, TileTypeInformation, CoordAccess};
use crate::models::{MapInfo, TileType};

mod models;
mod builders;
mod graphs;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn clone_map(original_map: &Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    original_map
        .iter()
        .map(|v| {
            v.iter()
                .map(|n| { n.shallow_clone() })
                .collect()
        })
        .collect()
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let file_result = fs::File::open("data.json");
    let file = match file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let json: JsonResult<MapInfo> = serde_json::from_reader(file);

    let parsed = match json {
        Ok(json) => json,
        Err(_) => return Err(io::Error::new(ErrorKind::AlreadyExists, "")),
    };

    let mut map: Vec<Vec<Rc<RefCell<Node>>>> = Vec::with_capacity(parsed.map.len());
    let mut original_map: Vec<Vec<Node>> = Vec::with_capacity(parsed.map.len());
    let mut x = 0;
    let mut y = 0;

    for row in parsed.map {
        let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(row.len());
        let mut original_nodes: Vec<Node> = Vec::with_capacity(row.len());
        for ttype in row {
            nodes.push(Rc::new(RefCell::new(Node::from_coords(x, y, ttype.clone()))));
            original_nodes.push(Node::from_coords(x, y, ttype));
            x += 1;
        }

        y += 1;
        x = 0;
        map.push(nodes);
        original_map.push(original_nodes);
    }

    fill_neighbours(&map);

    let start_node = find_start_node(&map);
    if start_node.is_none() {
        return Ok(());
    }

    let start = start_node.unwrap();
    let result = cutoff_builder(&start, &map, map.len() * map[0].len(), 8);

    let mut drawn_map = clone_map(&original_map);

    let mut towers: usize = 0;
    for coord in &result.best_towers[towers] {
        let mut node = drawn_map.get_mut(coord);
        node.ttype = TileType::Occupied;
    }


    terminal.draw(|f|{ui(f, &drawn_map)})?;
    loop {
        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }

            if let KeyCode::Left = key.code {
                if towers == 0 {
                    towers = result.best_towers.len() - 1;
                } else {
                    towers -= 1;
                }

                drawn_map = clone_map(&original_map);

                for coord in &result.best_towers[towers] {
                    let mut node = drawn_map.get_mut(coord);
                    node.ttype = TileType::Occupied;
                }
            } else if let KeyCode::Right = key.code {
                if  towers == result.best_towers.len() - 1{
                    towers = 0;
                } else {
                    towers += 1;
                }

                drawn_map = clone_map(&original_map);

                for coord in &result.best_towers[towers] {
                    let mut node = drawn_map.get_mut(coord);
                    node.ttype = TileType::Occupied;
                }
            }

            terminal.draw(|f|{ui(f, &drawn_map)})?;
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, map: &Vec<Vec<Node>>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size_orig = f.size();

    let y_max = map.len() as u32;
    let x_max = map[0].len();
    let max_s = std::cmp::min(size_orig.width, size_orig.height);

    let size = Rect::new(0, 0, x_max as u16 * 3, y_max as u16); // magic number

    // Surrounding block
    let block = Block::default();
    f.render_widget(block, size);

    let mut x = 0;
    let mut y = 0;

    let constraints: Vec<Constraint> = (0u32..y_max).map(|_|{Constraint::Ratio(1, y_max)}).collect();
    let chunks_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(size);

    for row in map {
        let x_max = row.len() as u32;
        let constraints: Vec<Constraint> = (0u32..x_max).map(|_|{Constraint::Ratio(1, x_max)}).collect();
        let chunks_h = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(chunks_v[y]);

        for cell in row {
            let color = cell.color();
            let block = Block::default()
                .style(Style::default().bg(color));
             f.render_widget(block, chunks_h[x]);
            x += 1;
        }
        x = 0;
        y += 1;
    }
}
