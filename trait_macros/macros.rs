macro_rules! HelloWorld {
    () => {
        println!("Hello, World")
    };
    ("KUNAL") => {
        println!("Hello, Kunal")
    };
    ("KUNAL", "SINGH") => {
        println!("Hello, Kunal Singh")
    };
}

/* literal, ident, expr are called code fragments
 */
macro_rules! add {
    ($a: literal, $b: literal) => {
        println!("{}", $a + $b)
    };
    ($a: literal, $b: literal, $c: literal) => {
        println!("{}", $a + $b + $c)
    };
    ($a: literal, $b: literal, $c: literal, $d: literal) => {
        println!("{}", $a + $b + $c + $d)
    };
}

macro_rules! hello_x {
    // $s is capture
    // ($s: literal) => {
    //     println!("{}", $s)
    // };
    // ($s: ident) => {
    //     println!("{}", $s)
    // }
    ($s: expr) => {
        println!("{}", $s)
    };
}

// macro_rules! set {

// }

fn main() {
    HelloWorld!();
    HelloWorld!("KUNAL");
    HelloWorld!("KUNAL", "SINGH");

    add!(1, 2);
    add!(1, 2, 3);
    add!(1, 2, 3, 4);

    let x = 10;
    dbg!(x);
    hello_x!("app");
    hello_x!(x);
}
