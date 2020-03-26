pub fn if_statement() {
    let temp = 35;

    if temp > 30 { // note no parenthesises
        println!("hot!");
    } else if temp < 10 {
        println!("cold!");
    } else {
        println!("normal");
    }
    // if is an expression
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    // inline if
    println!("it is {}",
    if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    // inline if statement
    println!("it is {}",
    if temp > 20 {
        if temp > 30 {"very hot"} else {"hot"}
    } else if temp < 10 {"cold"} else {"OK"});

}

pub fn while_loop() {
    let mut x = 1;
    while x < 1000 {
        x <<= 1;
        if x == 64 {
            continue;
        }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true
        y <<= 1;
        println!("y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

pub fn for_loop() {
    for x in 1..11 { // from 1 to 10 it is range
        if x == 3 {
            continue;
        }
        if x == 8 {
            break;
        }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn match_statement() {
    let country_code = 7; // from 1 to 999
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "unknown", // 999 is included in range
        _ => "invalid"
    };
    println!("the country with code {} is {}", country_code, country);
}