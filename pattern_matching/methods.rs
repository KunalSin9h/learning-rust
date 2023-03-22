enum Color {
    Red,
    Green,
    Blue,
    Custom { red: u8, green: u8, blue: u8 },
}

/*
 * Self = Type
 * self = self: Self
 * .......self: Type of (a: Type)
 */

impl Color {
    // fn get_rgb(clr: Color) -> (u8, u8, u8) {
    // self = self: Color
    fn get_rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => return (0, 255, 0),
            Color::Green => return (0, 255, 0),
            Color::Blue => return (0, 255, 0),
            Color::Custom { red, green, blue } => return (red, green, blue),
        }
    }
    // fn new(r: u8, g: u8, b: u8) -> Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        if r == 255 && g == 0 && b == 0 {
            return Color::Red;
        } else if g == 255 && r == 0 && b == 0 {
            return Color::Green;
        } else if b == 255 && r == 0 && g == 0 {
            return Color::Blue;
        } else {
            return Color::Custom {
                red: r,
                green: g,
                blue: b,
            };
        }
    }
}

// struct User {
//     username: &str,
//     password: &str,
// }

// impl User {
//     fn new(username: &str, password: &str) -> User {
//         return User {
//             username, // username: username -> username, [just like javascript]
//             password,
//         }
//     }

//     fn get(user: User) -> () {
//         println!("===================================");
//         println!("Username: {}", user.username);
//         println!("Password: {}", user.password);
//         println!("===================================");
//     }
// }

fn main() {
    let black_color = Color::new(0, 0, 0);
    let white_color: Color = Color::new(255, 255, 255);
    let (x, y, z) = Color::get_rgb(black_color);
    let (p, q, r) = Color::get_rgb(white_color);
    println!("{}, {}, {}", x, y, z);
    println!("{}, {}, {}", p, q, r);

    let plum = Color::new(211, 160, 221);

    let (a, b, c) = plum.get_rgb();
    println!("{}, {}, {}", a, b, c);

    // let kunal = User::new("kunalsin9h", "pass");
    // User::get(kunal);
}
