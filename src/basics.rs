use std::mem;

const SOME_CONST:u8 = 44; // have to explicitly specify type; no fixed address
static mut GLOBAL:u8 = 33;

pub fn types() {
    let a:u8 = 123; // immutable variable by default
    println!("a = {}", a);

    let mut b:i8 = 12; // need to be explicit with mut when declaring
    println!("b = {}", b);
    b = 127;
    println!("b = {}", b);

    let mut c = 123456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -123;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z:isize = 123; // isize, usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up = {} bytes, {}-bit os", z, size_of_z, size_of_z * 8);

    let d = 'k'; // size is 4 bytes :O
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double by default
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

pub fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4;
    // cannot a++ :(
    a += 1;
    println!("a = {}", a);
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3); // no power operator
    println!("a_cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // explicit call to power to int
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("b_cubed = {}", b_cubed);
    println!("b_to_pi = {}", b_to_pi);

    // bitwise | & ^ ! are there << >> as well
    let c = 1 | 2;
    println!("c = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical > <= >= == normal stuff
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("pi_less_4 = {}", pi_less_4);

}

pub fn scope_shadowing() {
    let a = 123;
    let a = 12; // redeclaration is ONLY WARNING, strange
    {
        let b = 456;
        println!("b = {}", b);
        let a = 44;
        println!("inside a = {}", a);
    }
    println!("outside a = {}", a);
}

pub fn global() {
    unsafe {
        println!("{}, {}", SOME_CONST, GLOBAL);
        GLOBAL += 1;
        println!("{}", GLOBAL);
    }
}