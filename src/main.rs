use image::{Rgb, Rgba, Pixel, RgbImage};
use std::vec::Vec;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use rand;

static WIDTH: u16 = 16001;
static HEIGHT: u16 = 16001;

struct Grid {
    positions: Vec<Vec<Position>>,
    width:u16,
    height: u16,   
    path_color: Rgb<u8>,
    background_color: Rgb<u8>
}

impl Grid {
    
    pub fn init(width: u16, height: u16, path_color: Rgb<u8>, back_color: Rgb<u8>) -> Self {
        let grid = Grid::generate_initial(width, height, path_color, back_color);
        Grid {
            positions: grid,
            width, 
            height, 
            path_color, 
            background_color: back_color
        }
    }

    pub fn generate_initial(width: u16, height: u16, path_color: Rgb<u8>, back_color: Rgb<u8>) -> Vec<Vec<Position>> {
        let mut overall: Vec<Vec<Position>> = Vec::new();
        for row in 0..width {
            let mut curr: Vec<Position> = Vec::new();
            for col in 0..height {
                let mut p: Position = Position::init(row, col);
                if row == 0 && col == 1 || row == 1 && col == 1 || row == width-1 && col == height-2 {
                    p.set_color(back_color.clone());
                } else {
                    p.set_color(path_color.clone());
                }
                curr.push(p);
            }
            overall.push(curr);
        }
        overall
    }

    pub fn save_image(&self) {
        let mut img = RgbImage::new(self.width as u32, self.height as u32);
        for row in 0..self.width {
            for col in 0..self.height {
                let pixel = self.positions.get(row as usize).unwrap().get(col as usize).unwrap().tile_color;
                img.put_pixel(col as u32, row as u32, pixel);
            }
        }
        
        img.save("maze.png");
    }
}

struct Position {
    row: u16,
    col: u16,
    tile_color: Rgb<u8>,
    neighbors: Vec<(u16, u16)>, // Use the coordinates+visited instead of object 
    walls: Vec<(u16, u16)>, // Use the coordinates+visited instead of object
    visited: bool
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

    pub fn find_neighbors(&mut self) {
        if self.row.checked_sub(2).is_some() {
            self.neighbors.push((self.row - 2, self.col));
        }
        if (self.col + 2) <= (WIDTH-1) {
            self.neighbors.push((self.row, self.col + 2));
        }
        if (self.row + 2) <= (HEIGHT-1) {
            self.neighbors.push((self.row + 2, self.col));
        }
        if self.col.checked_sub(2).is_some() {
            self.neighbors.push((self.row, self.col - 2));
        }
    }

    pub fn find_walls(&mut self) {
        if self.row.checked_sub(1).is_some() {
            self.walls.push((self.row - 1, self.col));
        }
        if (self.col + 1) <= (WIDTH-1) {
            self.walls.push((self.row, self.col + 1));
        }
        if (self.row + 1) <= (HEIGHT-1) {
            self.walls.push((self.row + 1, self.col));
        }
        if self.col.checked_sub(1).is_some() {
            self.walls.push((self.row, self.col - 1));
        }
    }


    pub fn set_color(&mut self, color: Rgb<u8>) {
        self.tile_color = color;
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn get_neighbor_pos(&self, grid: &Grid) -> Option<(u16, u16)> {
        let mut choose: Vec<(u16, u16)> = Vec::new();
        //Find all unvisited neighbors
        for n in self.neighbors.as_slice() {
            if !grid.positions.get(n.0 as usize).unwrap().get(n.1 as usize).unwrap().is_visited() {
                choose.push(n.clone());
            }
        }

        //Choose random neighbor
        if choose.len() == 0 {
            return None;
        }

        let index: u8 = rand::random::<u8>() % (choose.len() as u8);
        match choose.get(index as usize) {
            None => {
                None
            },
            Some(pos) => {
                Some(pos.to_owned())
            }
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Position: \nrow: {}, col: {}, has visited: {}, tile color: {:?}\nneighbors: {:?}\nwalls: {:?}\n",
               self.row, self.col, self.visited, self.tile_color, self.neighbors, self.walls)
    }
}

fn wall_pos(current_pos: (u16, u16), neighbor_pos: (u16, u16)) -> Option<(u16, u16)> {
    let coldif: i32 = (neighbor_pos.1 as i32) - (current_pos.1 as i32);
    let rowdif: i32 = (neighbor_pos.0 as i32) - (current_pos.0 as i32);

    if rowdif.abs() == 2 {
        return Some((((current_pos.0 as i32) + rowdif/2) as u16, current_pos.1));
    } else if coldif.abs() == 2 {
        return Some((current_pos.0, ((current_pos.1 as i32) + (coldif/2)) as u16));
    }
    None
}

fn main() {
    let back_rgb: &Rgb<u8> = Pixel::from_slice(&[0; 3]);
    let path_rbg: &Rgb<u8> = Pixel::from_slice(&[255; 3]);
    let mut grid: Grid = Grid::init(WIDTH, HEIGHT, path_rbg.to_rgb(), back_rgb.to_rgb());

    //Find neighbors
    for row in grid.positions.iter_mut() {
        for val in row.iter_mut() {
            val.find_neighbors();
            val.find_walls();
            //print!("{}\n", val);
        }
        //println!();
    }

    let mut backtrack: Vec<(u16,u16)> = Vec::new();
    let mut current: (u16, u16) = (1,1);
    backtrack.push(current);
    grid.positions.get_mut(current.0 as usize).unwrap().get_mut(current.1 as usize).unwrap().visited = true;
    let mut iter_num: u64 = 0;

    println!("Starting the backtracking algorithm");
    while !backtrack.is_empty() {
        iter_num += 1;
        let current_pos: &Position = grid.positions.get(current.0 as usize).unwrap().get(current.1 as usize).unwrap();
        //println!("{}", current_pos);

        match current_pos.get_neighbor_pos(&grid) {
            Some(pos) => {
                {
                    let neighbor: &mut Position = grid.positions.get_mut(pos.0 as usize).unwrap().get_mut(pos.1 as usize).unwrap();
                    //println!("Found neighbor at {:?}", pos);

                    neighbor.visited = true;
                    neighbor.tile_color = back_rgb.clone();
                }
                backtrack.push(pos);
                let wall_pos = wall_pos(current, pos).unwrap();
                let wall: &mut Position = grid.positions.get_mut(wall_pos.0 as usize).unwrap().get_mut(wall_pos.1 as usize).unwrap();
                wall.tile_color = back_rgb.clone();
                current = pos;
            },
            None => {
                current = backtrack.pop().unwrap();
                //println!("Moving backwards to {:?}", current);
            }
        }
        //println!("Backtrack has {} elements", backtrack.len());
    }

    println!("Took {} iterations to run algorithm", iter_num);

    grid.save_image();
}













