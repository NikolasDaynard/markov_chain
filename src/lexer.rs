use nannou::color::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, *};
use regex::Regex;

use crate::Pattern;

pub fn parse_file(arg: String) -> Vec<Pattern>{
    let file =  fs::read_to_string(arg).expect("Unable to read file");
    let parts = file.lines();
    let mut patterns: Vec<Pattern> = vec![];

    for part in parts {
        let pattern = convert_string_to_pattern(part.to_string());
        if pattern.is_some() {
            patterns.push(pattern.unwrap());
        }
    }

    return vec![
        // Rule: Replace WHITE space surrounded by BLACKs into a BLACK (to form cave structure)
        Pattern::new(vec![BLACK, WHITE, WHITE], vec![BLACK, GRAY, GRAY]), // random walk
        Pattern::new(vec![GRAY, WHITE], vec![WHITE, GRAY]), // expand
        Pattern::new(vec![GRAY], vec![BLUE]), // homogonize
        Pattern::new(vec![BLUE, WHITE], vec![RED, BLUE]), // expand again

        Pattern::new(vec![RED, WHITE], vec![BLACK, PINK]), // make more
        Pattern::new(vec![PINK, RED], vec![BLUE, BLUE]), // Expand pink doorways
        Pattern::new(vec![PINK, BLUE], vec![BLUE, BLUE]), // Expand pink doorways

        Pattern::new(vec![BLACK], vec![BLUE]), // cleanup
    ];
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
    
    let mut sides = code.split("="); // split l & r based on the =

    println!("code is: {}", code);
    assert_eq!(sides.clone().count(), 2, "Sides of expression \"{}\" are not equal to 2", input);
    for side in sides.clone() {
        println!("side: {}", side);
    }
    let colorMap = HashMap(String, Pattern);
    assert_eq!(sides.clone().nth(0).unwrap().len(), sides.clone().nth(1).unwrap().len(), "Sides of expression \"{}\" are not equal in length", input);

    None
}