#![allow(dead_code)]
#![allow(unused_variables)]

mod stack_heap;
mod control_flow;
mod basics;
mod data_structures;
mod advanced_ds;
mod functions;

fn main() {
    basics::types();
    basics::operators();
    basics::scope_shadowing();
    basics::global();
    stack_heap::stack_heap();
    control_flow::if_statement();
    control_flow::while_loop();
    control_flow::for_loop();
    control_flow::match_statement();
    data_structures::structures();
    data_structures::enums();
    data_structures::option();
    data_structures::array();
    data_structures::vector();
    advanced_ds::slices();
    advanced_ds::strings();
    advanced_ds::tuples();
    advanced_ds::pattern_matching();
    advanced_ds::generics();
    functions::basic();
    functions::methods();
    functions::closures(); // lambdas
    functions::high_order();
    functions::traits();

}
