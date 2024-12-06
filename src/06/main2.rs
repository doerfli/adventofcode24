use std::{fs, vec};
use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut map = get_input();
    let mut guard_x = 0 as usize;
    let mut guard_y = 0 as usize;
    let mut guard_orientation = Orientation::N;
    let len_y = map.len();
    let len_x: usize = map[0].len();
    // initialize with empty string vec
    let mut modified_map_loop = 0;
    let mut steps: Vec<(usize,usize,Orientation)> = vec![];
    
    (guard_x, guard_y, guard_orientation) = find_guard(&map);
    println!("Guard is at: ({}, {}), facing: {}", guard_x, guard_y, guard_orientation);

    let solved_map = solve_guard_track(&map);

    println!("Solved map:");
    for y in 0..len_y {
        for x in 0..len_x {
            print!("{}", solved_map[y][x]);
        }
        println!("");
    }


    for y in 0..len_y {
        for x in 0..len_x {
            if map[y][x] == '#' {
                continue;
            }
            if map[y][x] == '^' {
                continue;
            }
            if !has_passed(&solved_map, &x, &y, &len_x, &len_y) {
                continue;
            }

            // println!("Checking: ({}, {})", x, y);
            let is_loop = check_guard_loop(modify_map(&map, &x, &y), &guard_x, &guard_y, &guard_orientation);
            if (is_loop) {
                println!("Loop detected at: ({}, {})", x, y);
                modified_map_loop += 1;
            } else {
                println!("No loop detected at: ({}, {})", x, y);
            }
        }
    }

    println!("Modified map loop: {}", modified_map_loop);
} 

fn has_passed(map: &Vec<Vec<char>>, x: &usize, y: &usize, len_x: &usize, len_y: &usize) -> bool {
    if *y > 0 && map[*y-1][*x] == 'X' {
        return true;
    }
    if *y < (*len_y - 1) && map[*y+1][*x] == 'X' {
        return true;
    }
    if *x > 0 && map[*y][*x-1] == 'X' {
        return true;
    }
    if *x < (*len_x - 1) && map[*y][*x+1] == 'X' {
        return true;
    }
    return false;
}

fn check_guard_loop(mut map: Vec<Vec<char>>, aguard_x: &usize, aguard_y: &usize, aguard_orientation: &Orientation) -> bool {
    let mut guard_x = *aguard_x;
    let mut guard_y = *aguard_y;
    let mut guard_orientation = *aguard_orientation;
    let len_y = map.len();
    let len_x: usize = map[0].len();
    // initialize with empty string vec
    let mut steps: Vec<(usize,usize,Orientation)> = vec![];
    
    // println!(".");

    let mut is_loop = false;

    while true {
        let old_x = guard_x;
        let old_y = guard_y;

        map[old_y][old_x] = 'X';
        steps.push((guard_x, guard_y, guard_orientation));
        // println!("Guard is at: ({}, {}), facing: {}, step: {}", guard_x, guard_y, guard_orientation, steps.len());
        
        let object_ahead = get_object_ahead(&map, &guard_x, &guard_y, &guard_orientation, &len_x, &len_y);

        match object_ahead {
            'A' => {
                break;
            }
            '#' => {
                match guard_orientation {
                    Orientation::N => {
                        guard_orientation = Orientation::E;
                    }
                    Orientation::E => {
                        guard_orientation = Orientation::S;
                    }
                    Orientation::S => {
                        guard_orientation = Orientation::W;
                    }
                    Orientation::W => {
                        guard_orientation = Orientation::N;
                    }
                }
            }
            _ => {
                match guard_orientation {
                    Orientation::N => {
                        guard_y -= 1;
                    }
                    Orientation::E => {
                        guard_x += 1;
                    }
                    Orientation::S => {
                        guard_y += 1;
                    }
                    Orientation::W => {
                        guard_x -= 1;
                    }
                }
            }
        }

        if steps.contains(&(guard_x, guard_y, guard_orientation)) {
            is_loop = true;
            break;
        }

        // map[guard_y][guard_x] = 'G';
    }

    return is_loop;
}

fn solve_guard_track(amap: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = amap.clone();
    let mut guard_x = 0 as usize;
    let mut guard_y = 0 as usize;
    let mut guard_orientation = Orientation::N;
    let len_y = map.len();
    let len_x: usize = map[0].len();
    // initialize with empty string vec
    let mut steps: Vec<(usize,usize,Orientation)> = vec![];
    
    (guard_x, guard_y, guard_orientation) = find_guard(&map);
    // println!(".");

    let mut is_loop = false;

    while true {
        let old_x = guard_x;
        let old_y = guard_y;

        map[old_y][old_x] = 'X';
        steps.push((guard_x, guard_y, guard_orientation));
        // println!("Guard is at: ({}, {}), facing: {}, step: {}", guard_x, guard_y, guard_orientation, steps.len());
        
        let object_ahead = get_object_ahead(&map, &guard_x, &guard_y, &guard_orientation, &len_x, &len_y);

        match object_ahead {
            'A' => {
                break;
            }
            '#' => {
                match guard_orientation {
                    Orientation::N => {
                        guard_orientation = Orientation::E;
                    }
                    Orientation::E => {
                        guard_orientation = Orientation::S;
                    }
                    Orientation::S => {
                        guard_orientation = Orientation::W;
                    }
                    Orientation::W => {
                        guard_orientation = Orientation::N;
                    }
                }
            }
            _ => {
                match guard_orientation {
                    Orientation::N => {
                        guard_y -= 1;
                    }
                    Orientation::E => {
                        guard_x += 1;
                    }
                    Orientation::S => {
                        guard_y += 1;
                    }
                    Orientation::W => {
                        guard_x -= 1;
                    }
                }
            }
        }

        if steps.contains(&(guard_x, guard_y, guard_orientation)) {
            is_loop = true;
            break;
        }

        map[guard_y][guard_x] = 'G';
    }

    return map;
}

fn modify_map(map: &Vec<Vec<char>>, x: &usize, y: &usize) -> Vec<Vec<char>> {
    let mut modified_map = map.clone();
    modified_map[*y][*x] = '#';
    return modified_map;
}

fn get_object_ahead(map: &Vec<Vec<char>>, guard_x: &usize, guard_y: &usize, guard_orientation: &Orientation, len_x: &usize, len_y: &usize) -> char {
    match guard_orientation {
        Orientation::N => {
            if *guard_y == 0 {
                return 'A';
            }
            let new_y = guard_y - 1;
            return map[new_y][*guard_x];
        }
        Orientation::E => {
            if *guard_x == *len_x - 1 {
                return 'A';
            }
            let new_x = guard_x + 1;
            return map[*guard_y][new_x];
        }
        Orientation::S => {
            if *guard_y == *len_y - 1 {
                return 'A';
            }
            let new_y = guard_y + 1;
            return map[new_y][*guard_x];
        }
        Orientation::W => {
            if *guard_x == 0 {
                return 'A';
            }
            let new_x = guard_x - 1;
            return map[*guard_y][new_x];
        }
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize, Orientation) {
    let len_y = map.len();
    let len_x = map[0].len();
    let mut guard_x = 0;
    let mut guard_y = 0;
    let mut guard_orientation = Orientation::N;

    for y in 0..len_y {
        for x in 0..len_x {
            if map[y][x] == '^' {
                guard_x = x;
                guard_y = y;
                guard_orientation = Orientation::N;
                break;
            } else if map[y][x] == '>' {
                guard_x = x;
                guard_y = y;
                guard_orientation = Orientation::E;
                break;
            } else if map[y][x] == 'v' {
                guard_x = x;
                guard_y = y;
                guard_orientation = Orientation::S;
                break;
            } else if map[y][x] == '<' {
                guard_x = x;
                guard_y = y;
                guard_orientation = Orientation::W;
                break;
            }
        }
    }

    return (guard_x, guard_y, guard_orientation);
}


fn get_input() -> Vec<Vec<char>> {
    let content = fs::read_to_string("src/06/input.txt")
    // let content = fs::read_to_string("src/06/testinput.txt")
        .expect("Should have been able to read the file");
    return content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

#[derive(Copy, Clone, PartialEq)]
enum Orientation {
    N,
    E,
    S,
    W
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::N => write!(f, "North"),
            Orientation::E => write!(f, "East"),
            Orientation::S => write!(f, "South"),
            Orientation::W => write!(f, "West"),
        }
    }
}
