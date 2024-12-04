use std::fs;
use regex::Regex;

fn main() {
    let matrix = get_input();
    println!("{:?}", matrix);
    let len_y = matrix.len();
    let len_x = matrix[0].len();

    let mut found_xmas = 0;

    for y in 0..len_y {
        for x in 0..len_x {
            if matrix[y][x] == 'X' {
                if is_valid("XMAS", &matrix, x, y, 1, 0, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, 1, 1, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, 0, 1, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, -1, 1, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, -1, 0, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, -1, -1, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, 0, -1, len_x, len_y) {
                    found_xmas += 1;
                }
                if is_valid("XMAS", &matrix, x, y, 1, -1, len_x, len_y) {
                    found_xmas += 1;
                }
            }
        }
    }
    
    println!("The number of XMAS found is: {}", found_xmas);
} 

fn is_valid(word: &str, matrix: &Vec<Vec<char>>, start_x: usize, start_y: usize, dx: i32, dy: i32, len_x: usize, len_y: usize) -> bool {
    let len = word.len();
    let mut x = start_x as i32;
    let mut y = start_y as i32;

    for i in 0..len {
        if x < 0 || x >= len_x as i32 || y < 0 || y >= len_y as i32 {
            return false;
        }
        if matrix[y as usize][x as usize] != word.chars().nth(i).unwrap() {
            return false;
        }
        x += dx;
        y += dy;
    }
    return true;
}

fn get_input() -> Vec<Vec<char>> {
    let content = fs::read_to_string("src/04/input.txt")
    // let content = fs::read_to_string("src/04/testinput.txt")
        .expect("Should have been able to read the file");
    return content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

