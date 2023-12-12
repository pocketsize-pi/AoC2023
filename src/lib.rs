

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub enum InputType {
    Sample,
    Data,
    Manual,
}

pub fn read_input(day: u8, input: InputType, manual_name: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {

    let file_name = match input {
        InputType::Sample => format!("src/day{:02}_sample.txt",day),
        InputType::Data=> format!("src/day{:02}_input.txt",day),
        InputType::Manual => format!("src/{}", manual_name),
    };
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let words: Vec<String> = line.split_whitespace()
            .map(|s| s.to_string())
            .collect();

        data.push(words);
    }

    // println!("{:?}", data);
    Ok(data)
}

pub fn manhattan_distance((x1, y1): (&i32, &i32), (x2, y2): (&i32, &i32)) -> i32 {
    let x_diff = (x1 - x2).abs();
    let y_diff = (y1 - y2).abs();

    x_diff + y_diff
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Point {
    pub c: i32,
    pub r: i32,
}


// column is x, row is y
pub fn get_full_adjacent_values (pt: &Point, grid: &Vec<Vec<char>>) -> (Vec<char>,Vec<Point>) {
    let width = grid[0].len();
    let height = grid.len();
    let mut return_values: Vec<char> = Vec::new();
    let mut return_coords: Vec<Point> = Vec::new();
    let my_range : Vec<i32> = vec!(-1, 0, 1);
    for r_i in &my_range {
        for c_i in &my_range {
            let new_r = pt.r + r_i;
            let new_c = pt.c + c_i;
            if ! ((new_r < 0) | (new_r >= height as i32) | (new_c < 0) | (new_c >= width as i32)) {
                return_values.push(grid[new_r as usize][new_c as usize]);
                return_coords.push(Point { c: new_c, r: new_r});
            }

        }
    }
    (return_values, return_coords)
}

pub fn get_direct_adjacent_values (pt: &Point, grid: &Vec<Vec<char>>) -> (Vec<char>,Vec<Point>) {
    let width = grid[0].len();
    let height = grid.len();
    let mut return_values: Vec<char> = Vec::new();
    let mut return_coords: Vec<Point> = Vec::new();
    let my_range : Vec<i32> = vec!(-1, 1);
    for r_i in &my_range {
        let new_r = pt.r + r_i;
        if ! ((new_r < 0) | (new_r >= height as i32)) {
            return_values.push(grid[new_r as usize][pt.c as usize]);
            return_coords.push(Point { c: pt.c, r: new_r});
        }
    }
    for c_i in &my_range {
        let new_c = pt.c + c_i;
        if ! ((new_c < 0) | (new_c >= width as i32)) {
            return_values.push(grid[pt.r as usize][new_c as usize]);
            return_coords.push(Point { c: new_c, r: pt.r });
        }

    }
    (return_values, return_coords)
}

pub fn get_direct_adjacent_values_offset (pt: &Point, grid: &Vec<Vec<char>>) -> (Vec<char>,Vec<Point>) {
    let width = grid[0].len();
    let height = grid.len();
    let mut return_values: Vec<char> = Vec::new();
    let mut return_coords: Vec<Point> = Vec::new();
    let my_range : Vec<i32> = vec!(-1, 1);
    for r_i in &my_range {
        let new_r = pt.r + r_i;
        if ! ((new_r < 0) | (new_r >= height as i32)) {
            return_values.push(grid[new_r as usize][pt.c as usize]);
            return_coords.push(Point { c: 0, r: r_i.clone()});
        }
    }
    for c_i in &my_range {
        let new_c = pt.c + c_i;
        if ! ((new_c < 0) | (new_c >= width as i32)) {
            return_values.push(grid[pt.r as usize][new_c as usize]);
            return_coords.push(Point { c: c_i.clone(), r: 0});
        }

    }
    (return_values, return_coords)
}

pub const NORTH: Point = Point { r:-1, c:0};
pub const SOUTH: Point = Point { r:1, c:0};
pub const EAST: Point = Point { c:1, r:0};
pub const WEST: Point = Point { c:-1, r:0};
pub const STAY: Point = Point { c:0, r:0};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub fn move_point(point: &Point, direction: Direction) -> Point {
    let mut new_point : Point = Point { c: point.c, r: point.r };
    match direction {
        Direction::North => new_point.r -= 1,
        Direction::South => new_point.r += 1,
        Direction::East => new_point.c += 1,
        Direction::West => new_point.c -= 1,
        Direction::NorthEast => {new_point.r -= 1; new_point.c += 1;}
        Direction::NorthWest => {new_point.r -= 1; new_point.c -= 1;}
        Direction::SouthEast => {new_point.r += 1; new_point.c += 1;}
        Direction::SouthWest => {new_point.r += 1; new_point.c -= 1;}
    }
    new_point
}

pub fn move_by_offset(point: &Point, offset: &Point) -> Point {
    let mut new_point : Point = Point { c: point.c, r: point.r };
    match offset {
        Point { r:-1, c:0} => new_point = move_point(point, Direction::North),
        Point { r:1, c:0} => new_point = move_point(point, Direction::South),
        Point { c:1, r:0}  => new_point = move_point(point, Direction::East),
        Point { c:-1, r:0}  => new_point = move_point(point, Direction::West),
        _=> new_point = Point { c: point.c, r: point.r },
    }
    new_point
}

pub fn find_point_direction(origin: &Point, destination: &Point) -> Direction {
    let difference = Point{ r: (destination.r - origin.r).signum(),
        c: (destination.c - origin.c).signum()};
    let mut direction = Direction::North;
    match difference {
        Point { r:-1, c:0} => direction = Direction::North,
        Point { r:1, c:0} => direction = Direction::South,
        Point { c:1, r:0}  => direction = Direction::East,
        Point { c:-1, r:0}  => direction = Direction::West,
        _=> direction = Direction::North, // not the best default, maybe todo
    }
    direction
}

pub fn get_opposite_direction (direction: &Direction) -> Direction {
    match direction {
        Direction::North => {Direction::South}
        Direction::South => {Direction::North}
        Direction::East => {Direction::West}
        Direction::West => {Direction::East}
        Direction::NorthEast => {Direction::SouthWest}
        Direction::NorthWest => {Direction::SouthEast}
        Direction::SouthEast => {Direction::NorthWest}
        Direction::SouthWest => {Direction::NorthEast}
    }
}

// Set up for the FW algorithm
// pub const INF: i32 = i32::MAX / 2;
// let mut graph = vec![vec![INF; num_vertices]; num_vertices];
// then add the rest of the

// Create a matrix A0 of dimension n*n where n is the number of vertices. The row and the column are indexed as i and j respectively. i and j are the vertices of the graph.
// Each cell A[i][j] is filled with the distance from the ith vertex to the jth vertex. If there is no path from ith vertex to jth vertex, the cell is left as infinity.
// eg
// G = [[0, 3, INF, 5],
//      [2, 0, INF, 4],
//      [INF, 1, 0, INF],
//      [INF, INF, 2, 0]]
pub fn floyd_warshall(mut graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let num_vertices = graph.len();

    for k in 0..num_vertices{
        for i in 0..num_vertices {
            for j in 0..num_vertices {
                graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
            }
        }
    }
    graph
}

pub fn string_to_u32 (input: &String) -> u32 {
    input.parse::<u32>().unwrap()
}

pub fn string_to_u64 (input: &String) -> u64 {
    input.parse::<u64>().unwrap()
}

pub fn string_to_i64 (input: &String) -> i64 {
    input.parse::<i64>().unwrap()
}

pub fn string_to_chars(input: &String) -> Vec<char> {
    input.chars().collect()
}