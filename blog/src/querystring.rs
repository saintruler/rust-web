use std::str;
use std::collections::HashMap;
use std::num::ParseIntError;


pub fn parse_qs(query: String) -> HashMap<String, String> {
    let query = percent_decode(query);
    let mut map = HashMap::new();
    for pair in query.split('&') {
        let pair = pair.split('=').collect::<Vec<&str>>();
        map.insert(pair[0].to_string(), pair[1].to_string());
    }
    return map;
}

fn percent_decode(s: String) -> String {
    let len = s.len();
    let mut s = s.chars();
    let mut result = String::new();
    let mut decode = String::new();

    let mut i = 0; 
    while i < len {
        let cur = s.nth(0).unwrap();
        if cur == '%' {
            decode.push(s.nth(0).unwrap());
            decode.push(s.nth(0).unwrap());
            i += 3;
            continue;
        }
        if decode.len() != 0 {
            match decode_hex(&decode) {
                Ok(v) => result.push_str(str::from_utf8(&v).unwrap()),
                Err(_) => ()
            };
            decode = String::new();
        }

        result.push(cur);
        i += 1;
    }
    return result;
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
