use std::fs;
use regex::Regex;

fn main() {
    let contents = get_input();
    // println!("{}", contents);
    let mut sum_of_mul = 0;
    let pattern = Regex::new(r"(do\(\)|don\x27t\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut add_to_sum = true;

    for line in contents.lines() {    
        for (cap) in pattern.captures_iter(line) {
            let text = cap.get(0).unwrap();
            println!("text: {}", text.as_str());
            if (text.as_str() == "don't()") {
                add_to_sum = false;
            } else if (text.as_str() == "do()") {
                add_to_sum = true;
            } else if add_to_sum {
                let x1 = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let x2 = cap.get(3).unwrap().as_str().parse::<i32>().unwrap();
                
                let sum = x1 * x2;
                sum_of_mul += sum;
            }
        }
    }

    println!("The sum of the products is: {}", sum_of_mul);
} 

fn get_input() -> String {
    return fs::read_to_string("src/03/input.txt")
    // return fs::read_to_string("src/03/testinput2.txt")
        .expect("Should have been able to read the file");
}

