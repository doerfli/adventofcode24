use std::fs;

fn main() {
    let contents = get_input();
    // println!("{}", contents);
    let mut valid_reports = 0;

    for line in contents.lines() {
        // split and map to i32
        // let tokens = line.split(" ").collect::<Vec<&i32>>();
        // let tokens = line.split(" ").collect::<Vec<&str>>();
        let tokens = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        
        if is_safe(tokens) {
            valid_reports += 1;
            print!("Valid");
        } else {
            print!("Invalid");
        }

        println!("");
    }

    println!("The number of valid reports is: {}", valid_reports);
}

fn get_input() -> String {
    return fs::read_to_string("src/02/input.txt")
    // return fs::read_to_string("src/02/testinput.txt")
        .expect("Should have been able to read the file");
}

fn is_safe(tokens: Vec<i32>) -> bool {
    let mut is_increasing = false;

    for i in 1..tokens.len() {
        let curr = tokens[i];
        let prev = tokens[i - 1];
        print!("{} - {} / ", prev, curr);
        if curr == prev {
            return false;
        }

        let diff = (curr - prev).abs();
        if diff > 3 {
            return false;
        }

        if i == 1 {
            is_increasing = curr > prev;
        } else {
            if is_increasing && curr < prev {
                return false;
            } else if !is_increasing && curr > prev {
                return false;
            }
        }
    }

    return true;
}