enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        } 
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

// fn print_color(color: Color) -> () {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Yellow => println!("yellow"),
//     }
// }


struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number (usize),
    String (String),
    Custom (Custom),
}

fn append(items: &mut Vec<Item>) -> () {
    items.push(Item::Number(10));
}

fn main(){
    let v: Color = Color::Green;
    let is_green = v.is_green();
    dbg!(is_green);

    let mut v: Vec<Item> = Vec::new();
    append(&mut v);

    match &v[0] {
        Item::Number(x) if *x as i32 == 10 => println!("5 + 5 = {}", x),
        Item::String(x) => println!("{}", x),
        _ => unreachable!(), // default case
    }

}
