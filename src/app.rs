use crate::models::{self, Element, get_grid_position};
use std::collections::HashMap;

pub struct App {
    pub elements: Vec<Element>,
    pub grid: HashMap<(usize, usize), usize>,
    pub cursor: (usize, usize),
    pub is_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let elements = models::load_dataset();
        let grid = elements
            .iter()
            .enumerate()
            .map(|(i, e)| (get_grid_position(e.atomic_number), i))
            .collect();

        Self {
            elements,
            grid,
            cursor: (0, 0),
            is_quit: false,
        }
    }
    pub fn get_selected_element(&self) -> Option<&Element> {
        self.grid.get(&self.cursor).map(|&i| &self.elements[i])
    }
    pub fn step(&mut self, dr: isize, dc: isize) {
        let (mut r, mut c) = (self.cursor.0 as isize, self.cursor.1 as isize);
        loop {
            r += dr;
            c += dc;
            if !(0..10).contains(&r) || !(0..18).contains(&c) {
                return;
            }
            if self.grid.contains_key(&(r as usize, c as usize)) {
                self.cursor = (r as usize, c as usize);
                return;
            }
        }
    }
}
