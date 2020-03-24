extern crate phrases;

use phrases::greetings::french;

pub fn phrase() {
    println!("English: {}, {}",
             phrases::greetings::english::hello(),
             phrases::greetings::english::goodbye());
    println!("French: {}, {}",
             french::hello(),
             french::goodbye());
}