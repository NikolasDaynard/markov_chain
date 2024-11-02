use nannou::color::*;

use std::collections::HashMap;
use std::fs::{self};
use regex::Regex;

use crate::Pattern;

pub fn parse_file(arg: String) -> Vec<Pattern>{
    let file =  fs::read_to_string(arg).expect("Unable to read file");
    let parts = file.lines();
    let mut patterns: Vec<Pattern> = vec![];

    for part in parts {
        let pattern = convert_string_to_pattern(part.to_string());
        if pattern.is_some() {
            pattern.clone().unwrap().print();
            patterns.push(pattern.unwrap());
            println!("added pattern: ");
        }
    }

    return patterns;
}


pub fn convert_string_to_pattern(input: String) -> Option<Pattern> {
    // Regular expression to capture code and comment separately
    let re = Regex::new(r"(?P<code>.*?)(?P<comment>//.*)?$").unwrap();
    
    let mut code = "";
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            // Capture the code and comment as separate groups
            code = caps.name("code").map_or("", |m| m.as_str()).trim();
        }
    }
    if code == "" {
        return None
    }

    let mut code = code.to_string();
    if code.starts_with('(') {
        println!("it's a random one");
        code = code.replace(['(', ')'], "");
    }
    
    let sides = code.split("="); // split l & r based on the =

    println!("code is: {}", code);
    assert_eq!(sides.clone().count(), 2, "Sides of expression \"{}\" are not equal to 2", input);
    for side in sides.clone() {
        println!("side: {}", side);
    }
    assert_eq!(sides.clone().nth(0).unwrap().len(), sides.clone().nth(1).unwrap().len(), "Sides of expression \"{}\" are not equal in length", input);
    
    let mut color_map: HashMap<String, rgb::Rgb<encoding::Srgb, u8>> = HashMap::new();
    color_map.insert("k".to_string(), BLACK);
    color_map.insert("w".to_string(), WHITE);
    color_map.insert("g".to_string(), GRAY);
    color_map.insert("r".to_string(), RED);
    color_map.insert("p".to_string(), PINK);
    color_map.insert("b".to_string(), BLUE);
    
    let mut left_side_color: Vec<rgb::Rgb<encoding::Srgb, u8>> = vec![];
    
    for char in sides.clone().nth(0).unwrap().chars() {
        left_side_color.push(*color_map.get(&char.to_string()).unwrap());
    }

    let mut right_side_color: Vec<rgb::Rgb<encoding::Srgb, u8>> = vec![];
    
    for char in sides.clone().nth(1).unwrap().chars() {
        right_side_color.push(*color_map.get(&char.to_string()).unwrap());
    }
    right_side_color.reverse(); // it's flipped for no reason

    return Some(Pattern::new(left_side_color, right_side_color));

    // None
}