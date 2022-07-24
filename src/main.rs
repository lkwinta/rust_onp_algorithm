use std::collections::LinkedList;

fn is_operator(c : &str) -> bool {
    return c == "-" || c == "+" || c == "/" || c == "*";
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

fn main() {
    let onp_string = String::from("2 7 + 3 / 14 3 - 4 * + 2 /");
    //let onp_array : str[] = onp_string.split_whitespace();

    let mut stack: LinkedList<String> = LinkedList::new();

    for single  in onp_string.split_whitespace(){
        if is_operator(single) == true {
            let right = stack.pop_back().unwrap().parse::<f64>().unwrap();
            let left = stack.pop_back().unwrap().parse::<f64>().unwrap();

            let result = count(left, right, single);
            println!("{} {} {} = {}", left, single, right, result);

            stack.push_back(result.to_string());
        } else {
            println!("-----> {}", single.to_string());
            stack.push_back(single.to_string());
        }

        println!("{} -> {}",single, is_operator(single));
    }

    println!("Result: {}", stack.pop_back().unwrap());
}
