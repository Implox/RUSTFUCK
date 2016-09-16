mod interpreter;

use std::io::prelude::*;
use std::fs::File;
use std::env;

use interpreter::Machine;

fn main() {
    let file_name = {
        let prog_args: Vec<String> = env::args().collect();
        prog_args[1].clone()
    };

    let code = {
        let mut file = File::open(file_name).ok().unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        s
    };
    let tokens: Vec<char> = code.chars().collect();
    let mut machine = Machine::new(tokens);
    machine.execute(false);
}
