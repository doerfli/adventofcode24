use std::fs;

fn main() {
    let content = get_input();
    let mut total_calibration_result = 0;

    for line in content.lines() {
        let tokens = line.split(":").collect::<Vec<&str>>();
        let target = tokens[0].parse::<i64>().unwrap();
        let numbers = tokens[1].trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        println!("Target: {}, Numbers: {:?}", target, numbers);

        let is_success = calculate(target, &numbers, 0, 0);

        if is_success {
            println!("Success");
            total_calibration_result += target;
            continue;
        }

        for i in 0..numbers.len() - 1 {
            let mut modified_numbers = numbers.clone();
            let new_element = format!("{}{}", numbers[i], numbers[i + 1]).parse::<i64>().unwrap();
            modified_numbers.splice(i..i + 2, vec![new_element]);
            println!("Modified Numbers: {:?}", modified_numbers);

            let is_success = calculate(target, &modified_numbers, 0, 0);
            if is_success {
                println!("Success");
                total_calibration_result += target;
                break;
            }
        }
    }

    println!("The total calibration result is: {}", total_calibration_result);
} 

fn calculate(target: i64, numbers: &Vec<i64>, sum: i64, index: usize) -> bool {
    if index == numbers.len() {
        return sum == target;
    }

    if sum > target {
        return false;
    }

    let current = numbers[index];
    return  calculate(target, numbers, sum + current, index + 1) || calculate(target, numbers, sum * current, index + 1);
}


fn get_input() -> String {
    // return fs::read_to_string("src/07/input.txt")
    return fs::read_to_string("src/07/testinput.txt")
        .expect("Should have been able to read the file");
}
