use std::fs;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut map = get_input();
    let mut guard_x = 0 as usize;
    let mut guard_y = 0 as usize;
    let mut guard_orientation = Orientation::N;
    let len_y = map.len();
    let len_x = map[0].len();
    
    (guard_x, guard_y, guard_orientation) = find_guard(&map);
    println!("Guard is at: ({}, {}), facing: {}", guard_x, guard_y, guard_orientation);

    while true {
        let old_x = guard_x;
        let old_y = guard_y;

        map[old_y][old_x] = 'X';

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

        map[guard_y][guard_x] = 'G';
    }

    let mut count = 0;

    for y in 0..len_y {
        for x in 0..len_x {
            print!("{}", map[y][x]);
            if map[y][x] == 'X' {
                count += 1;
            }
        }
        println!();
    }

    println!("Count: {}", count);
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
    // let content = fs::read_to_string("src/06/input.txt")
    let content = fs::read_to_string("src/06/testinput.txt")
        .expect("Should have been able to read the file");
    return content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

#[derive(Copy, Clone)]
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
