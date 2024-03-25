

mod parse_prop;
mod parse;

const USAGE: &str = "usage: smt <filename>";

fn main() {
    let filename = std::env::args().nth(1).expect(USAGE);
    let read_err = format!("Problem reading {filename}");
    let contents = std::fs::read_to_string(filename).expect(&read_err);
    //parse_prop::parse(&contents);
}
