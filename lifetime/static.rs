use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement {}", ann);
    if x.len() > y.len() {
        return x;
    }
    return y;
}

fn something<T>(x: i32, y: i32, z: T) -> T
where
    T: Display,
{
}
