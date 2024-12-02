use std::fs;

fn main() {
    let (mut list1, mut list2) = get_lists();
    let mut how_far = 0;
    list1.sort();
    list2.sort();

    for i in 0..list1.len() {
        // let diff = (list1[i] - list2[i]).abs();
        // println!("{} - {} = {}", list1[i], list2[i], diff);
        // how_far += diff;
        
        // find number of occurences of i in list2
        let mut count = 0;
        for j in 0..list2.len() {
            if list2[j] == list1[i] {
                count += 1;
            }
        }

        how_far += count * list1[i];
    }

    println!("The answer is: {}", how_far);
}

fn get_lists() -> (Vec<i32>, Vec<i32>) {
    // let contents = fs::read_to_string("01/testinput.txt")
    let contents = fs::read_to_string("src/01/input.txt")
        .expect("Should have been able to read the file");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        // print!("{line}");
        let tokens = line.split("   ").collect::<Vec<&str>>();
        // for token in tokens {
        //     let num = token.parse::<i32>().unwrap();
        // }
        let num1 = tokens[0].parse::<i32>().unwrap();
        let num2 = tokens[1].parse::<i32>().unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    (list1, list2)
}