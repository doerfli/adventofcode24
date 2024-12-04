use std::fs;

fn main() {
    let matrix = get_input();
    println!("{:?}", matrix);
    let len_y = matrix.len();
    let len_x = matrix[0].len();

    let mut found_mas = 0;

    for y in 1..len_y - 1 {
        for x in 1 .. len_x - 1 {
            if matrix[y][x] == 'A' {
                // extract the X chars as text and match against valid MAS x MAS variants
                let diagonals = get_diagonals(&matrix, x, y, len_x, len_y);
                println!("Diagonals: {}", diagonals);
                if (diagonals == "MASMAS") {
                    found_mas += 1;
                } else if (diagonals == "MASSAM") {
                    found_mas += 1;
                } else if (diagonals == "SAMMAS") {
                    found_mas += 1;
                } else if (diagonals == "SAMSAM") {
                    found_mas += 1;
                }
            }
        }
    }
    
    println!("The number of XMAS found is: {}", found_mas);
} 

fn get_diagonals(matrix: &Vec<Vec<char>>, x: usize, y: usize, len_x: usize, len_y: usize) -> String {
    let mut diagonals = "".to_owned();
    diagonals.push(matrix[y-1][x-1]);
    diagonals.push(matrix[y][x]);
    diagonals.push(matrix[y+1][x+1]);
    diagonals.push(matrix[y-1][x+1]);
    diagonals.push(matrix[y][x]);
    diagonals.push(matrix[y+1][x-1]);
    return diagonals;
}

fn get_input() -> Vec<Vec<char>> {
    let content = fs::read_to_string("src/04/input.txt")
    // let content = fs::read_to_string("src/04/testinput.txt")
        .expect("Should have been able to read the file");
    return content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
}

