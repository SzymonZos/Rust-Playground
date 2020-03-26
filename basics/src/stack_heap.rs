use std::mem;

struct Point {
    x:f64,
    y:f64,
    z:f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0, z: 0.0}
}

pub fn stack_heap() {
    let p1 = origin();
    let p2 = Box::new(origin()); // heap allocation
    // println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p1 takes up {} bytes", mem::size_of_val(&p2));
    println!("p1 takes up {} bytes", mem::size_of_val(&*p2));

    let p3 = *p2;
    println!("{}", p3.x);
}