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
        let num_tokens = tokens.len();
        
        let invalid_level = is_safe(&tokens);

        if invalid_level == -1 {
            valid_reports += 1;
            print!("Valid");
        } else {
            let mut valid_without_level = -1;
            for i in 0..num_tokens {
                let mut tokens_m = tokens.clone();
                tokens_m.remove(i);
                let invalid_level = is_safe(&tokens_m);
                if invalid_level == -1 {
                    valid_without_level = i as i32;
                    break;
                }
            }
            
            if valid_without_level > -1 {
                valid_reports += 1;
                print!("Valid after removing level {}", valid_without_level);
            } else {
                print!("Invalid");
            }
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

fn is_safe(tokens: &Vec<i32>) -> i32 {
    let mut is_increasing = false;
    let mut allow_one_exception = true;

    for i in 1..tokens.len() {
        let curr = tokens[i];
        let prev = tokens[i - 1];
        print!("{} - {} / ", prev, curr);
        if curr == prev {
            return i as i32;
        }

        let diff = (curr - prev).abs();
        if diff > 3 {
            return i as i32;
        }

        if i == 1 {
            is_increasing = curr > prev;
        } else {
            if is_increasing && curr < prev {
                return i as i32;
            } else if !is_increasing && curr > prev {
                return i as i32;
            }
        }
    }

    return -1;
}