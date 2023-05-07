trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    x: f64,
    y: f64,
    height: f64,
    width: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.height * self.width;
    }
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return std::f64::consts::PI * self.radius * self.radius;
    }
}

fn print_area(something: &impl Area) {
    println!("{}", something.area());
}

fn main(){
    let rect = Rectangle { x: 1f64, y: 1f64, height: 1f64, width: 1f64 };

    let c = Circle { x: 1f64, y: 1f64, radius: 1f64 };

    print_area(&rect);
    print_area(&c);
}
