fn use_slice(slice: &mut[i32]) {
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

pub fn slices() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    use_slice(&mut data);
    println!("{:?}", data);

}

pub fn strings() {
    let s: &'static str = "hello there"; // &str = string slice, something like string_view
    // s = "abc"; cannot, immutable
    // let h = s[0]; cannot either
    for c in s.chars().rev() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // String -> heap allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // &str <> String
    let u:&str = &letters;

    // concatenation
    // String + str

    let z = letters + "abc"; // implicit move
    // let y = letters + &letters;

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32) { // tuple as return type
    (x + y, x * y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("combined: {:?}", combined);
    println!("last elem = {}", (combined.1).1);

    let ((c, d), (e, f)) = combined;
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);
    let wat = (42,); // tuple of 1 element, need that extra ,
    println!("{:?}", wat);
}

fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        z @ 9..=11 => "lots of",
        12 => "a dozen",
        _ if (x % 2 == 0) => "some", // no implicit bool conversion
        _ => "a few"
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (1, 7);
    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (ref x, 0) => println!("y axis, x = {}", x),
        (_, y) => println!("(?, {})", y)
    }
}

struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

pub fn generics() {
    let a:Point<f32> = Point{x: 0.0, y: 4.0}; // explicit type
    let b = Point{x: 1.2, y: 3.4}; // type deduction

    let my_line = Line{start: a, end: b};
}