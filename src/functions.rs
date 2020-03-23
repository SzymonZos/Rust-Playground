fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 {
    x * y // return without semicolon at the end
}

pub fn basic() {
    print_value(32);
    let mut z = 1;
    increase(&mut z);
    print_value(z);
    print_value(product(3, 4));
}

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn methods() {
    let p = Point{x: 3.0, y: 4.0};
    let p2 = Point{x: 5.0, y: 10.0};
    let my_line = Line{start: p, end: p2};
    println!("length = {}", my_line.len());
}

fn say_hello() {
    println!("Hello!");
}

pub fn closures() { // lambdas
    let sh = say_hello;
    sh();

    let plus_one = |x:i32| -> i32 {x + 1};
    print_value(plus_one(10));

    let two = 2;
    let plus_two = |x| { // [capture] is by reference by default
      let mut z = x;
        z += two;
        z
    };
    print_value(plus_two(3));
    // let borrow_two = &mut two; // cannot do that unless two is mut

    // T: by value
    // T&
    // &mut &
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    print_value(f);
}

pub fn high_order() {
    let limit = 500;
    let mut sum = 0;

    for i in 0.. { // Dunno why this is there
        let isq = i * i;
        if isq > limit {
            break;
        } else if isq % 2 == 0 {
            sum += isq;
        }
    }
    print_value(sum);

    let sum2 = // Ridiculous, strange and hard to read
        (0..).map(|x| x*x)
             .take_while(|&x| x < limit)
             .filter(|x| x % 2 == 0)
             .fold(0, |sum, x| sum + x);
    print_value(sum2);
}

trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

struct Cat {
    name: &'static str
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{name}
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;
        }
        result
    }
}

pub fn traits() {
    let h = Human{name: "John"};
    let c = Cat{name: "Catty"};
    h.talk();
    c.talk();
    let h2 = Human::create("George");
    h2.talk();
    let human: Human = Animal::create("Polymorphic");
    human.talk();

    let a = vec![1, 2, 3];
    println!("sum = {}", a.sum());
}