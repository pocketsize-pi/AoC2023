use std::cmp::max;
use std::collections::HashMap;
// use AoC2023::{InputType, string_to_chars, Point, get_direct_adjacent_values};
use AoC2023::*;
use std::collections::VecDeque;

pub fn day10(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 10");


    let data = AoC2023::read_input(10, input_type, manual_name)?;

    // for row in &data {
    //     println!("{:?}", row);
    // }


    // this is a BFS algorithm
    // we visit the top node, and add all its friends to the queue
    // we keep a visited list
    // and we'll want to keep a dictionary of properties
    // and at some point we'll need to create a base implementation we can roll out every time

    enum Pipes {
        Vertical,   // |
        Horizontal, // -
        NE,         // L
        NW,         // J
        SW,         // 7
        SE,         // F
        Ground,     // .
        Start,      // S
    }

    fn symbol_to_pipe (symbol: &char) -> Pipes {
        match symbol {
            '|' => Pipes::Vertical,
            '-' => Pipes::Horizontal,
            'L' => Pipes::NE,
            'J' => Pipes::NW,
            '7' => Pipes::SW,
            'F' => Pipes::SE,
            'S' => Pipes::Start,
            '.' => Pipes::Ground,
            _=> Pipes::Ground,
        }
    }

    fn is_actual_pipe (symbol : &char) -> bool {
        match symbol {
        '|' => true,
        '-' => true,
        'L' => true,
        'J' => true,
        '7' =>true,
        'F' => true,
        'S' => true,
        '.' => false,
        _=> false,
        }
    }

    // let NORTH: Point = Point {y:1, x:0};
    // let SOUTH: Point = Point {y:-1, x:0};
    // let EAST: Point = Point {x:1, y:0};
    // let WEST: Point = Point {x:-1, y:0};
    // pub const STAY: Point = Point {x:0, y:0};

    fn symbol_connects (coord: &Point, offset: &Point, pipe: &Pipes) -> (bool, Point) {
        let mut connects = false;
        let mut destination = Point { c:coord.c, r:coord.r };
        match pipe {
            Pipes::Vertical => {
                connects = (*offset == NORTH ) | (*offset== SOUTH);
                destination = move_by_offset(coord, offset);
            }
            Pipes::Horizontal => {
                connects = (*offset == EAST ) | (*offset== WEST);
                destination = move_by_offset(coord, offset);
            }
            Pipes::NE => {
                connects = (*offset == SOUTH) | (*offset == WEST);
                // destination = move_point(coord, Direction::NorthEast);
                destination = move_by_offset(coord, offset);
            }

            Pipes::NW => {
                connects = (*offset == SOUTH) | (*offset == EAST);
                // destination = move_point(coord, Direction::NorthWest);
                destination = move_by_offset(coord, offset);
            }
            Pipes::SW => {
                connects = (*offset == NORTH) | (*offset == EAST);
                // destination = move_point(coord, Direction::SouthWest);
                destination = move_by_offset(coord, offset);
            }
            Pipes::SE => {
                connects = (*offset == NORTH) | (*offset == WEST);
                // destination = move_point(coord, Direction::SouthEast);
                destination = move_by_offset(coord, offset);
            }
            Pipes::Ground => { connects =false}
            Pipes::Start => {connects = true } // ?
        }
        (connects, destination)
    }

    fn get_next_pipe (point: &Point, parent: &Point, pipe_map: &Vec<Vec<char>>) -> Point {
        let mut destination = Point { c:point.c, r:point.r };
        let pipe = symbol_to_pipe(&pipe_map[point.r as usize][point.c as usize]);
        // println!("pipe: {}", &pipe_map[point.r as usize][point.c as usize]);
        // println!("direction: {:?}", find_point_direction(parent, point));
        match pipe {
            Pipes::Vertical => {
                destination = move_point(point, (find_point_direction(parent,point)));
            }
            Pipes::Horizontal => {
                destination = move_point(point, (find_point_direction(parent,point)));
            }
            Pipes::NE => {
                if find_point_direction(parent, point) == Direction::South {
                    destination = move_point(point, Direction::East);
                }
                else { destination = move_point(point, Direction::North); }
            }
            Pipes::NW => {
                if find_point_direction(parent, point) == Direction::South {
                    destination = move_point(point, Direction::West);
                }
                else { destination = move_point(point, Direction::North); }
            }
            Pipes::SW => {
                if find_point_direction(parent, point) == Direction::North {
                    destination = move_point(point, Direction::West);
                }
                else { destination = move_point(point, Direction::South); }
            }
            Pipes::SE => {
                if find_point_direction(parent, point) == Direction::North {
                    destination = move_point(point, Direction::East);
                }
            else { destination = move_point(point, Direction::South); }
            }
            Pipes::Ground => {}
            Pipes::Start => {}
        }
        destination
    }

    // turn into chars
    let mut pipe_map = Vec::new();
    let mut start_coords = Point { c: 0, r: 0}; // as row, column, so j, i
    for i  in 0..data.len() {
        // let row = &data[i];
        let my_row = string_to_chars(&data[i][0]);
        pipe_map.push(my_row.clone()); // cloning here to keep it afterwards
        if my_row.contains(&'S') {
            start_coords = Point { c:  my_row.iter().position(|&x| x == 'S').unwrap() as i32, r: i as i32};
        }
    }
    // println!("{:?}", &pipe_map);
    // println!("{:?}", &start_coords);

    let mut nodes_list: VecDeque<Point>  = VecDeque::new();
    let mut visited_nodes = Vec::new();
    let mut node_distance = HashMap::new();
    let mut node_parent = HashMap::new();

    // find all nodes adjacent to S, and check if they are connected to us
    // column is x, row is y

    let (adjacent_values, adjacent_offset) = get_direct_adjacent_values_offset(&start_coords,&pipe_map);
    // println!("adjacent values{:?}", &adjacent_values);
    // println!("{:?}", &adjacent_offset);
    visited_nodes.push(start_coords);
    for i in 1..adjacent_values.len() {
        let symbol = symbol_to_pipe(&adjacent_values[i]);
        let offset = &adjacent_offset[i];
        let (connects, new_point) = symbol_connects(&start_coords, offset, &symbol);

        if connects {
            // add to list
            nodes_list.push_back(new_point);
            node_distance.insert(new_point, 1);
            node_parent.insert(new_point, start_coords);

        }
    }
    // println!("nodes list{:?}", nodes_list);

    let mut back_round = false;

    while !back_round {
        // println!("loop");
        let current_option = nodes_list.pop_front();
        if current_option.is_none() {
            back_round = true;
        }
        else {
            let current_node = current_option.unwrap();
            visited_nodes.push(current_node);
            // println!("current node {:?}", current_node);
            // println!("&node_parent[&current_node] node {:?}", &node_parent[&current_node]);

            // each pipe only has one option, really
            let new_point = get_next_pipe(&current_node, &node_parent[&current_node], &pipe_map);
            // println!("new_point {:?}", new_point);
            // println!("visited_nodes list{:?}", visited_nodes);

            if !visited_nodes.contains(&new_point) {

                nodes_list.push_back(new_point);
                node_distance.insert(new_point, node_distance[&current_node] +1);
                node_parent.insert(new_point, current_node);
                // println!("adding node {:?}, node_distance {:?}; parent {:?}, parent distance {:?}",
                //          &new_point, node_distance[&new_point], current_node, node_distance[&current_node]);
            }

        }



    }

    println!("Part 1: {:?}", node_distance.values().max().unwrap());
    //13623 answer is too high
    // I'm not surprised it's all gone wrong, my directions are all over the place
    println!("Length of visited: {}", visited_nodes.len());
    println!("Part 1 hack: {}", visited_nodes.len()/2);
    // 6812 is right! So my algorithm BFS wasn't quite working right, but I was hitting all the nodes
    // and the right node is the one that is furthest on a circular loop

    // ok, let's see about this stupid part 2

    // println!("visited_nodes list{:?}", visited_nodes);
    let mut clean_map: Vec<Vec<char>> = Vec::new();
    for r_i  in 0..pipe_map.len() {
        let mut temp_row = Vec::new();
        for c_i in 0..pipe_map[0].len() {
            let temp_point = Point{r: r_i as i32, c:c_i as i32};
            // println!("{:?}", temp_point);
            if visited_nodes.contains(&temp_point) {
                // keep point
                temp_row.push(pipe_map[r_i][c_i].clone());
            }
            else {
                // replace with .
                temp_row.push(' ');
            }
        }
        clean_map.push(temp_row);
    }

    for row in &clean_map {
        let s: String = row.into_iter().collect();
        println!("{:?}", s);
    }
    println!();

    let mut inside_count = 0;
    let mut row_col_inside = Vec::new();
    // let mut col_row_inside = Vec::new();
    for r_i in 0..clean_map.len() {
        let row = clean_map[r_i].clone();
        let mut is_inside = false;
        for c_i in 0..row.len() {
            let col = &row[c_i];
            if is_actual_pipe(col) {
                is_inside = !is_inside;
            }
            else {
                if is_inside {
                    row_col_inside.push((r_i,c_i));
                    clean_map[r_i][c_i] = 'I';
                }
            }
        }
    }

    for c_i in 0..clean_map[0].len() {
        let mut col = Vec::new();
        for brute_row in &clean_map { // need a hashmap here next
            col.push(brute_row[c_i]);
        }
        // let col = &clean_map[c_i];
        let mut is_inside = false;
        // remove open top
        for r_i in 0..clean_map.len() {
            let row = &col[r_i];

            if is_actual_pipe(row) {
                is_inside = !is_inside;
            }
            else {
                if is_inside {
                    row_col_inside.push((r_i,c_i));
                    clean_map[r_i][c_i] = 'I';
                }
            }
        }
    }
    for row in &clean_map {
        let s: String = row.into_iter().collect();
        println!("{:?}", s);
    }
    println!();

    for c_i in 0..clean_map[0].len() {
        let mut col = Vec::new();
        for brute_row in &clean_map { // need a hashmap here next
            col.push(brute_row[c_i]);
        }
        // let col = &clean_map[c_i];
        let mut is_inside = false;
        // remove open top
        for r_i in 0..clean_map.len() {
            let row = &col[r_i];

            if is_actual_pipe(row) {
                break;
            }
            else {
                if row_col_inside.contains(&(r_i, c_i)) {
                    row_col_inside.retain(|value| *value != (r_i, c_i));
                    clean_map[r_i][c_i] = ' ';

                }
            }
        }
    }

    // and then the same but reversed
    for c_i in 0..clean_map[0].len() {
        let mut col = Vec::new();
        for brute_row in &clean_map {
            col.push(brute_row[c_i]);
        }
        // let col = &clean_map[c_i];
        let mut is_inside = false;
        // remove open top
        for r_i in (0..clean_map.len()).rev() {
            let row = &col[r_i];

            if is_actual_pipe(row) {
                break;
            }
            else {
                if row_col_inside.contains(&(r_i, c_i)) {
                    row_col_inside.retain(|value| *value != (r_i, c_i));
                    clean_map[r_i][c_i] = ' ';
                }
            }
        }
    }

    for row in &clean_map {
        let s: String = row.into_iter().collect();
        println!("{:?}", s);
    }
    println!();

    // fine, brute force
    // for entry in row_col_inside {
    //     if col_row_inside.contains(&entry) {
    //         inside_count += 1;
    //     }
    // }
    println!("Part 2: {}", row_col_inside.len());
    // 1593 is too high
    // 865 is still too high


    Ok(())
}