use regex::bytes::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

/*
Ported to Rust from https://github.com/utdemir/bencoder/
*/

/// TODO extend error types
#[derive(Debug)]
pub enum BencodeError {
    InvalidKey(String),
    Other(String),
}

impl From<String> for BencodeError {
    fn from(s: String) -> Self {
        BencodeError::Other(s)
    }
}

/// BencodeValues contain all possible bencoding data. Used in parsing .torrent files.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BencodeValue {
    String(String),                   // String
    Raw(Vec<u8>),                     // Raw bytes
    Integer(i64),                     // Integer values
    List(Vec<BencodeValue>),          // Nested lists
    Dictionary(HashMap<String, BencodeValue>), // Nested dictionaries
}

/// Deprecated
pub fn encode_str (inp: String) -> BencodeValue{
    let length : usize = inp.len();
    BencodeValue::String(format!("{}{}{}",length,":",inp))
}

/// Core decoder function. Recursively constructs BencodeValue structure,
/// uses byte regexes to find correct splices
fn decode_first(s: Vec<u8>) -> Result<(BencodeValue, Vec<u8>), BencodeError> {
    if s.is_empty(){
        return Err(BencodeError::Other("Empty Input".to_string()));
    }
    let s_slice: &[u8] = &s;
    if s_slice.starts_with(b"i") {
        let re = Regex::new(r"^i(-?\d+)e(.*)").unwrap();
        if let Some(cap) = re.captures(s_slice) {
            if let Some(val) = cap.get(1) {
                // Extract the matched number part
                let matched_str = std::str::from_utf8(val.as_bytes())
                    .map_err(|_| BencodeError::Other("Invalid UTF-8 in capture group".to_string()))?;
                let num: i64 = matched_str
                    .parse()
                    .map_err(|_| BencodeError::Other("Failed to parse i64".to_string()))?;
                // Get the position of the end of the first match (`i<digits>e`)

                let match_len = matched_str.len() + 2; // Length of the match, including 'i' and 'e'
                let next_index = cap.get(0).unwrap().start() + match_len;
                let remainder = s_slice[next_index..].to_vec();

                return Ok((BencodeValue::Integer(num), remainder));
            }
        }
    }

    else if s.starts_with(b"l") || s.starts_with(b"d") { // List or dictionary
        let mut l : Vec<BencodeValue> = Vec::new();
        let mut rest : Vec<u8> = s[1..].to_vec();
        while !rest.starts_with(b"e"){
            let result: Result<(BencodeValue, Vec<u8>), BencodeError> = decode_first(rest.clone());
            match result {
                Ok((elem, rest_temp)) => {
                    rest = rest_temp;
                    l.push(elem); // Builds list
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        rest = rest[1..].to_vec();
        // If it was a list
        if s.starts_with(b"l"){
            return Ok((BencodeValue::List(l), rest));
        }
        // If a dictionary
        else {
            let even_indices = l.iter().step_by(2); // Elements at even indices
            let odd_indices = l.iter().skip(1).step_by(2); // Elements at odd indices
            let map: HashMap<String, BencodeValue> = even_indices
                .zip(odd_indices)
                .map(|(key, value)| {
                    if let BencodeValue::Raw(key_raw) = key {
                        // Bencodings have valid strings as keys. If it fails to convert -> raise an error
                        match std::str::from_utf8(&key_raw) {
                            Ok(key_str) => Ok((key_str.to_string(), value.clone())),
                            Err(_) => Err(BencodeError::InvalidKey(format!("Invalid UTF-8 key: {:?}", key_raw))),
                        }
                    } else if let BencodeValue::String(key_str) = key {
                        // If the key is already a valid String, use it directly
                        Ok((key_str.clone(), value.clone()))
                    }
                    else {
                        Err(BencodeError::InvalidKey(format!("{:?}", key))) // Should not reach this point
                    }
                })
                .collect::<Result<HashMap<_, _>, _>>()?;
            return Ok((BencodeValue::Dictionary(map), rest));
        }
    }
    else if s.iter().any(|&c| (b'0'..=b'9').contains(&c)) {
        let re = Regex::new(r"^(\d+):").unwrap();  // String finder - #LEN#:STRING
        if let Some(captures) = re.captures(&s) {
            let length: usize = std::str::from_utf8(captures
                .get(1)
                .ok_or(BencodeError::Other("Failed in getting length of string".to_string()))?
                .as_bytes())  // Get the raw byte slice
                .map_err(|_| BencodeError::Other("Failed to convert bytes to string".to_string()))?
                .parse()  // Parse to usize
                .map_err(|_| BencodeError::Other("Failed to parse length".to_string()))?;

            let rest_i = captures
                .get(0)
                .ok_or(BencodeError::Other("Failed getting next string pointer".to_string()))? // Excessive errors?
                .end();

            let start = rest_i;
            let end = start + length;

            if end <= s.len() {
                let string_part = s[start..end].to_vec();
                let remainder = s[end..].to_vec();  // Collects relevant parts in Vec<u8>

                return match std::str::from_utf8(&string_part) {
                    Ok(valid_str) => Ok((BencodeValue::String(valid_str.to_string()), remainder)),
                    Err(_) => Ok((BencodeValue::Raw(string_part), remainder)),
                }
            }
        }
    }
    Err(BencodeError::Other("Broken Input".to_string())) // This may be reached if string splicing fails badly.
}

/// Wrapper for the decode_first machine. TODO pass the error on.
pub fn decode(inp: Vec<u8>) -> BencodeValue {
    let result: Result<(BencodeValue, Vec<u8>), BencodeError> = decode_first(inp.clone());
    match result {
        Ok((elem, _)) => {
            elem
        }
        Err(e) => {
            println!("Error decoding string {:?}",e);
            todo!()
        }
    }
}