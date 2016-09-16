mod interpreter;

use interpreter::*;

fn main() {
    let hello_world = "++++++++++[>+++++++>++++++++ \
                       ++>+++>+<<<<-]>++.>+.+++++++ \
                       ..+++.>++.<<+++++++++++++++. \
                       >.+++.------.--------.>+.>.";
    let mut machine = Machine::new(hello_world.chars().collect::<Vec<char>>());
    machine.execute(false);
}
