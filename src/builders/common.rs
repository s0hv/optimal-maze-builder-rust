use std::time;
use crate::graphs::utils::{Coords};

pub struct BuilderResult {
    pub duration: time::Duration,
    pub best_towers: Vec<Vec<Coords>>
}
