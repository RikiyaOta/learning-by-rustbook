use std::io;
use std::io::Write;

#[derive(Debug)]
struct Circle {
    radius: u32
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2    
    }
    
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }
}

mod server {
    pub fn echo() {
        println!("Server");
    }
}

mod client {
    pub fn echo() {
        println!("Client");
    }
}

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    
    let year = year.trim().parse::<u32>().unwrap();
    
    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year!", year);
    }
    
    let circle1 = Circle { radius: 10};
    println!("Circle1's diameter: {}", circle1.diameter());
    println!("Circle1: {:?}", circle1);
    
    let circle2 = Circle::small_circle();
    println!("Circle2's diameter: {}", circle2.diameter());
    
    server::echo();
    client::echo();
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}