use std::collections::LinkedList;
use crate::is_operator;

pub fn parse_to_onp(equation_string : String) -> String{
    let mut onp_string = String::new();

    let mut stack : LinkedList<&str> = LinkedList::new();

    for single in equation_string.split_whitespace(){
        if single == "("{
            stack.push_back(single);
        } else if single == ")"{
            while stack.back().unwrap().to_string() != "(" {
                onp_string.push_str(stack.pop_back().unwrap());
                onp_string.push(' ');
            }
            stack.pop_back();
        } else if is_operator::is_operator(single) {
            if single == "+" || single == "-"{
                while !stack.is_empty() && is_operator::is_operator(stack.back().unwrap()){
                    onp_string.push_str(stack.pop_back().unwrap());
                    onp_string.push(' ');
                }
            } else {
                while !stack.is_empty() && (stack.back().unwrap().to_string() == "*" || stack.back().unwrap().to_string() == "/" ) {
                    onp_string.push_str(stack.pop_back().unwrap());
                    onp_string.push(' ');
                }
            }
            stack.push_back(single);

        } else {
            onp_string.push_str(single);
            onp_string.push(' ');
        }
    }

    while !stack.is_empty() {
        onp_string.push_str(stack.pop_back().unwrap());
        onp_string.push(' ');
    }

    onp_string.pop();

    return onp_string;
}