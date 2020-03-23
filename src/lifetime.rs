pub fn ownership() {
    let v = vec![1, 2, 3];
    let v2 = v; // implicit move; no copy constructor
    println!("{:?}", v2);

    let u = 1;
    let u2 = u; // copy; primitive located on the stack
    println!("u = {}", u);

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x // inconvenient to return variable after taking its ownership
    };

    let vv = print_vector(v2);
    println!("{:?}", vv);
}

pub fn borrowing() {
    let print_vector = |x: &Vec<i32>| {
        println!("x[0] = {}", x[0]);
    };
    let v = vec![3, 2, 1];
    print_vector(&v);
    println!("v[0] = {}", v[0]);

    let mut a = 40;
    let b = &mut a;
    *b += 2;
    let c = &a;
    println!("a = {}", c);

    let mut z = vec![3, 2, 1];
    for i in &mut z {
        *i += 1;
        println!("i = {}", i);
    }
}