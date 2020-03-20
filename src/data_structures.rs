use std::mem;

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

pub enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    Cmyk{cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

pub fn structures() {
    let p = Point{x: 3.0, y: 4.0};
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point{x: 5.0, y: 10.0};
    let my_line = Line{start: p, end: p2};
}

pub fn enums() {
    let c = Color::Cmyk{cyan: 0, magenta: 0, yellow: 0, black: 255};
    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::Cmyk{cyan:_, magenta:_, yellow:_, black:255}=> println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => ()
    }
}

pub fn option() {
    let x = 2.4;
    let y = 0.0;
    // Option<T>: can contain Some(z) or None
    let result:Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };
    println!("{:?}", result);
    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}", x, y)
    }

    // if result is None then nothing happens
    if let Some(z) = result { println!("z = {}", z); }
}

// Displaying more complex types: {:?}; pretty-print: {:#?}
pub fn array() {
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array a: {:?}", a);
    println!("a has {} elements, first is {}",
        a.len(), a[0]);
    a[0] = 32;
    println!("a[0] = {}", a[0]);

    if a != [1, 2, 3, 4, 5] { // cannot compare to array with different size
        println!("does not match");
    }

    let b = [1u16; 10]; // b.len() == 10
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx:[[f32; 3]; 2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

pub fn vector() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}", a);
    a.push(44);
    println!("a = {:?}", a);

    // let idx:usize = 12; // runtime crash
    // a[idx] = 21;
    // println!("a[0] = {}", a[idx]);

    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a { println!("{}", x); }
    a.push(77);
    println!("{:?}", a);
    let last_elem = a.pop(); // Option
    println!("last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() { // prints backwards, if None then break
        println!("{}", x);
    }
}