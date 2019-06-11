use parser::parse;

fn main() {
    let result = parse("input");
    if result.is_ok() {
        println!("We are using the examplar code");
    } else {
        println!("We are using the participant code");
    }
}
