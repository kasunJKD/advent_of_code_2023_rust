use std::fs::File;
use std::io::{self};
use std::io::BufRead;

fn main()->io::Result<()>  {

    //12R 13G 14B
    //check each set if showed set item val > R or G or B 
    //invalid game
    //calc sum of game ids
    //[game:1,[r,g],[r,g,b]]

    
    let mut total = 0;

    let path = "input.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let result = parseline(line.as_str());
       

        // let game_number = match result.iter().find(|&x| x[0].0 == "Game") {
        //     Some(game) => game[0].1,
        //     None => 0,
        // };
        let mut found = false;
        let game_value = result.iter()
                         .find(|x| x[0].0 == "Game")
                         .map(|x| x[0].1)
                         .unwrap_or(0);

        for sub_array in result {
            for (color, count) in sub_array {
                if color == "red" && count > 12 {
                    found = true;
                    break;
                }
                else if color == "green" && count > 13 {
                    found = true;
                    break;
                }
                else if color == "blue" && count > 14 {
                    found = true;
                    break;
                }
            }
        }

        if found == true {
            println!("{:?}", game_value);
            total = total + game_value;
        }

    }   

    println!("{:?}", total);

    Ok(())



}

fn parseline(line: &str) -> Vec<Vec<(&str,i32)>> 
{
    let mut result = Vec::new();
    let parts = line.split(':').collect::<Vec<_>>();
    //println!("{:?}", parts);

    for part in parts {
        if part.trim().starts_with("Game") {
            if let Some((_, game_number)) = part.trim().split_once(' ') {                             
                    let val: i32 = game_number.parse().unwrap();
                    result.push(vec![("Game", val)]);               
            }
        } else { 
            let sparts = line.trim().split(';').collect::<Vec<_>>();
            for subpart in sparts{
                let color_counts = subpart.trim().split(',')
                .filter_map(|kv| {
                    let mut iter = kv.trim().split_whitespace();
                    
                    if let (Some(value), Some(key)) = (iter.next(), iter.next()) {
                        //println!("{:?}", key);
                        value.parse::<i32>().ok().map(|v| (key, v))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(&str, i32)>>();
            //println!("{:?}", color_counts);
            if !color_counts.is_empty() {
                result.push(color_counts);
            }
            }       
        }
    }
    result
}

