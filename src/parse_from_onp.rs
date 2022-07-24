use std::collections::LinkedList;
use crate::is_operator;

pub fn parse_from_onp(onp_string : String) -> f64 {
    let mut stack: LinkedList<String> = LinkedList::new();

    for single  in onp_string.split_whitespace(){
        if is_operator::is_operator(single) == true {
            let right = stack.pop_back().unwrap().parse::<f64>().unwrap();
            let left = stack.pop_back().unwrap().parse::<f64>().unwrap();

            let result = count(left, right, single);

            stack.push_back(result.to_string());
        } else {
            stack.push_back(single.to_string());
        }
    }

    return stack.pop_back().unwrap().parse::<f64>().unwrap();
}



fn count(l : f64, r : f64, o : &str) -> f64 {
    return match o {
        "+" => l + r,
        "-" => l - r,
        "*" => l * r,
        "/" => l / r,
        _ => 0.0,
    };
}
