enum Color {
    Green,
    Yellow,
    Red,
    Custom {
        red: u8,
        green: u8,
        yellow: u8,
    },
    Mixed (u8, u8, u8),
}

fn main(){

    let go: Color         = Color::Green;
    let slow_down: Color  = Color::Yellow;
    let stop: Color       = Color::Red;
    let black: Color      = Color::Custom {red: 0, green: 0, yellow: 0};
    let white: Color      = Color::Mixed(255, 255, 255);
    let off_white: Color  = Color::Mixed(255, 255, 200);

    print_color(go);
    print_color(slow_down);
    print_color(stop);
    print_color(black);
    print_color(white);
    print_color(off_white);

    // let color = go;
    
    // let current_color: Color = Color::Red;

    // match current_color {
    //     Color::Red => {
    //         println!("Its Red");
    //     }
    //     Color::Yellow => {
    //         println!("Its Yellow");
    //     }
    //     Color::Green => {
    //         println!("Its Green");
    //     }
    //     Color::Custom {red, green, yellow} => {
    //         println!("The colors are {}, {}, {}", red, green, yellow);
    //     }
    //     Color::Mixed(x, y, z) => {
    //         println!("The Mixed colors are {}, {}, {}", x, y, z);
    //     }
    // }
} 

fn print_color(clr: Color) {
    match clr {
        Color::Green => println!("Green"),
        Color::Yellow => println!("Yellow"),
        Color::Red => println!("Red"),
        Color::Custom{red, green, yellow} => println!("Custom {}, {}, {}", red, green, yellow),
        Color::Mixed(x, y, z) => println!("Mixed {}, {}, {}", x, y, z),
    }
}
