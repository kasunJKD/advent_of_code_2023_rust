use std::fs::File;
use std::io::{self};
use std::io::BufRead;
use regex::Regex;

fn main()->io::Result<()> {
   // 1abc2
//pqr3stu8vwx
//a1b2c3d4e5f
//treb7uchet
    //read line 
    //combine 2 numbers first and last
    //if 1 number exists 
    //same number twice

    let path = "src/input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    let re = Regex::new(r"\d").unwrap();
    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = re.find_iter(&line)
                                        .filter_map(|m| m.as_str().parse().ok())
                                        .collect();

        if numbers.len() >= 2 {
            let first_number = *numbers.get(0).unwrap_or(&0);
            let last_number = *numbers.get(numbers.len() - 1).unwrap_or(&0);

            let combined_number = first_number * 10 + last_number;
            println!("Combined number: {}", combined_number);
            total += combined_number;
        } else {
            let combined_number = numbers[0] * 10 + numbers[0];
            println!("Combined for 1: {}", combined_number);
            total += combined_number;
        }

        println!("Numbers in line: {:?}", numbers);
    }

    println!("Total: {}", total);

    Ok(())

}
