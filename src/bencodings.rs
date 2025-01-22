use serde_json;
use regex::Regex;
use serde_json::json;
use std::collections::HashMap;

/*
Ported to Rust from https://github.com/utdemir/bencoder/
*/

pub fn encode_str (inp: String) -> String{
    let length : usize = inp.len();
    format!("{}{}{}",length,":",inp)
}

fn decode_first(s: String) -> Option<(serde_json::Value, String)> {
    if s.starts_with("i") {
        let re = Regex::new(r"^i(-?\d+)e(.*)").unwrap();
        if let Some(cap) = re.captures(&s){
            if let Some(val) = cap.get(1){
                let num : i32 = val.as_str().parse().unwrap();
                let remainder : String = cap.get(2).unwrap().as_str().to_string();
                return Some((json!(num), remainder));
            }
        }
    }
    else if s.starts_with("l") || s.starts_with("d") {

        let mut l : Vec<serde_json::Value> = Vec::new();
        let mut rest : String = s[1..].to_string();
        while !rest.starts_with("e"){
            let Some((elem,rest_temp)) : Option<(serde_json::Value, String)> = decode_first(rest.clone()) else { todo!() };
            rest = rest_temp;
            l.push(elem)
        }
        rest = rest[1..].to_string();
        // If it was a list
        if s.starts_with("l"){
            return Some((json!(l), rest.to_string()));
        }
        // If a dictionary
        else {
            let even_indices = l.iter().step_by(2); // Elements at even indices
            let odd_indices = l.iter().skip(1).step_by(2); // Elements at odd indices

            let map: HashMap<serde_json::Value, serde_json::Value> = even_indices
                .zip(odd_indices)
                .map(|(i, j)| (i.clone(), j.clone()))
                .collect();
            return Some((json!(map), rest.to_string()));
        }
    }
    else if s.chars().any(|c| c.is_numeric()){
        let re = Regex::new(r"^(\d+):").unwrap();
        if let Some(captures) = re.captures(&s) {
            let length: usize = captures.get(1)?.as_str().parse().ok()?;
            let rest_i = captures.get(0)?.end();

            let start = rest_i;
            let end = start + length;

            if end <= s.len() {
                return Some((json!(&s[start..end]), (&s[end..]).parse().unwrap()));
            }
        }
    }
    None
}

pub fn decode(inp: String) -> serde_json::Value {
    let Some((result,_rest)) = decode_first(inp.clone()) else { todo!() };
    result
}