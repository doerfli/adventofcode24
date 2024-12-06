use std::fs;
use std::collections::HashMap;

fn main() {
    let (rules, updates) = get_input();
    
    for update in updates {
        let mut ordered_pages = Vec::new();
        for page in update {
            ordered_pages.push(page);
        }

    }
} 


fn get_input() -> (HashMap<i32, i32>, Vec<Vec<i32>>) {
    // let content = fs::read_to_string("src/05/input.txt")
    let content = fs::read_to_string("src/05/testinput.txt")
        .expect("Should have been able to read the file");
    
    let mut rules: HashMap<i32, i32> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut is_rules = true;

    for line in content.lines() {
        if line == "" {
            is_rules = false;
        } else if is_rules {
            //split line by | and insert into map
            let rule = line.split("|").collect::<Vec<&str>>();
            let key = rule[0].parse::<i32>().unwrap();
            let value = rule[1].parse::<i32>().unwrap();
            rules.insert(key, value);
        } else {
            let page = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            updates.push(page);
        }
    }

    return (rules, updates);
}

