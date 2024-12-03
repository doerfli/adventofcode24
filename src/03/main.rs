use std::fs;
use regex::Regex;

fn main() {
    let contents = get_input();
    // println!("{}", contents);
    let mut sum_of_mul = 0;
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for line in contents.lines() {
        let sum = 
            pattern.captures_iter(line).map(|x| x.extract())
                .map(|(_, [x1, x2])| x1.parse::<i32>().unwrap() * x2.parse::<i32>().unwrap())
                .sum::<i32>();


        sum_of_mul += sum;
    }

    println!("The sum of the products is: {}", sum_of_mul);
} 

fn get_input() -> String {
    return fs::read_to_string("src/03/input.txt")
    // return fs::read_to_string("src/03/testinput.txt")
        .expect("Should have been able to read the file");
}

