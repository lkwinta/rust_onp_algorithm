mod parse_from_onp;
mod parse_to_onp;
mod is_operator;


fn main() {
    let equation_string = String::from("( 1 + 2 ) * 4 + 5 - 3");
    println!("{}", parse_from_onp::parse_from_onp(parse_to_onp::parse_to_onp(equation_string)));
}
