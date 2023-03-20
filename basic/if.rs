fn main() {
    let cats = 10;

    let cat = if cats > 5 {
        "More then five"
    } else {
        "Less then five"
    };

    println!("{}", cat);

    let _ternary = if cats == 10 { "Yes" } else { "No" };

    let y: &str = "Kunal Singh";
    println!("{}", y);
}
