use std::fmt::Display;

struct User<'a> {
    name: &'a String,
    balance: &'a usize,
}

impl Display for User<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return writeln!(f, "User ( Name: {}, Balance: {} )", self.name, self.balance,);
    }
}

fn main() {
    let name = String::from("Kunal Singh");
    let balance = 10000;
    let x = User {
        name: &name,
        balance: &balance,
    };
    println!("{}", x);
    println!("{}", x.name);
}
