#![allow(dead_code)]
#![allow(unused_variables)]

mod stack_heap;
mod control_flow;
mod basics;
mod data_structures;

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
}
