use std::fs;

fn main() {
    let _input = fs::read_to_string("./input.txt").expect("File was not read");
    
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in _input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() == 2 {
            if let Ok(num1) = parts[0].parse::<i32>() {
                first_column.push(num1);
            } else {
                println!("Could not parse {} as i32!", parts[0]);
            }

            if let Ok(num2) = parts[1].parse::<i32>() {
                second_column.push(num2);
            } else {
                println!("Could not parse {} as i32!", parts[1]);
            }
        } else {
            println!("Skipping line: {:#?} not 2 parts", line);
        }
    }
    
    first_column.sort();
    second_column.sort();

    let mut difference: Vec<i32> = Vec::new();

    for i in 0..first_column.len() {
        let sum: i32 = first_column[i] - second_column[i];
        let absolute_sum: i32 = sum.abs();
        difference.push(absolute_sum);
    }
    
    let part_1_solution: i32 = difference.iter().sum();

    println!("Total sum: {}", part_1_solution);

    
    let mut similarity_sums: Vec<i32> = Vec::new();

    for item in first_column.iter() {
        let mut multiplier: i32 = 0;
        for entry in second_column.iter() {
            if item == entry {
                multiplier += 1;
            }
        }

        similarity_sums.push(item * multiplier);
    
    }

    let part_2_solution: i32 = similarity_sums.iter().sum();

    println!("Total similarity score: {}", part_2_solution);
}
