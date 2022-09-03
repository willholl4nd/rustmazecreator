use image::{Rgb, Rgba, Pixel};
use std::vec::Vec;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

struct Grid {
    grid: Vec<Vec<Rc<Position>>>,
    width: u16,
    height: u16,
    path_color: Rgb<u8>,
    back_color: Rgb<u8>,
}

impl Grid {

    pub fn init(width: u16, height: u16, path_color: Rgb<u8>, back_color: Rgb<u8>) -> Self {
        let grid = Grid::generate_initial(width, height, path_color, back_color);
        Grid {
            grid,
            width, 
            height, 
            path_color, 
            back_color
        }
    }

    pub fn generate_initial(width: u16, height: u16, path_color: Rgb<u8>, back_color: Rgb<u8>) -> Vec<Vec<Rc<Position>>> {
        let mut overall: Vec<Vec<Rc<Position>>> = Vec::new();
        for row in 0..width {
            let mut curr: Vec<Rc<Position>> = Vec::new();
            for col in 0..height {
                let mut p: Position = Position::init(row, col);
                if row == 0 && col == 1 || row == 1 && col == 1 || row == width-1 && col == height-2 {
                    p.set_color(back_color.clone());
                } else {
                    p.set_color(path_color.clone());
                }
                curr.push(Rc::new(p).clone());
            }
            overall.push(curr);
        }
        overall
    }

    pub fn find_surroundings(&mut self) {
        self.grid.iter_mut();
    }
}

struct Position {
    row: u16,
    col: u16,
    tile_color: Rgb<u8>,
    neighbors: Vec<Rc<Position>>,
    walls: Vec<Rc<Position>>,
    visited: bool
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Position: \nrow: {}\ncol: {}\nhas visited: {}\ntile color: {:?}\n", self.row, self.col, self.visited, self.tile_color)
    }
}

impl Position {

    pub fn init(row: u16, col: u16) -> Self {
        let tile_color = Rgb::from([0,0,0]);
        let neighbors = Vec::new();
        let walls = Vec::new();
        let visited = false;
        Position {
            row, 
            col,
            tile_color,
            neighbors,
            walls,
            visited
        }
    }

    pub fn set_color(&mut self, color: Rgb<u8>) {
        self.tile_color = color
    }

    pub fn find_neighbors(&self, grid: &Vec<Vec<Rc<Position>>>) -> Vec<Rc<Position>> {
        let mut ret: Vec<Rc<Position>> = Vec::new();
        if self.is_valid_row_index(self.row - 2) {
            let row: &Vec<Rc<Position>> = grid.get((self.row - 2) as usize).unwrap();
            let pos: Rc<Position> = row.get(self.col as usize).unwrap().clone();
            ret.push(pos)
        }
        if self.is_valid_col_index(self.col + 2) {
            let row: &Vec<Rc<Position>> = grid.get(self.row as usize).unwrap();
            let pos: Rc<Position> = row.get((self.col + 2) as usize).unwrap().clone();
            ret.push(pos)
        }
        if self.is_valid_row_index(self.row + 2) {
            let row: &Vec<Rc<Position>> = grid.get((self.row + 2) as usize).unwrap();
            let pos: Rc<Position> = row.get(self.col as usize).unwrap().clone();
            ret.push(pos)
        }
        if self.is_valid_col_index(self.col - 2) {
            let row: &Vec<Rc<Position>> = grid.get(self.row as usize).unwrap();
            let pos: Rc<Position> = row.get((self.col - 2) as usize).unwrap().clone();
            ret.push(pos)
        }
        ret
    }

    pub fn find_walls(&self, grid: &Vec<Vec<Position>>) -> Vec<Rc<Position>> {
        let ret: Vec<Rc<Position>> = Vec::new();


        ret
    }

    pub fn get_random_neighbor(&mut self) {

    }

    pub fn remove_wall(&mut self, neighbor: Position) {

    }

    pub fn has_neighbors(&self) -> bool {

        false
    }

    pub fn get_count(&self) -> u16 {

        0
    }

    pub fn is_valid_row_index(&self, row: u16) -> bool {

        false
    }

    pub fn is_valid_col_index(&self, col: u16) -> bool {

        false
    }

}


fn main() {
    let back_rgb: &Rgb<u8> = Pixel::from_slice(&[0; 3]);
    let path_rbg: &Rgb<u8> = Pixel::from_slice(&[255; 3]);
    let grid = Grid::init(10, 10, path_rbg.to_rgb(), back_rgb.to_rgb());
    for row in grid.grid {
        for val in row {
            //print!("({} {}) ", val.row, val.col);
            print!("{}\n", val);
        }
        println!();
    }
}
