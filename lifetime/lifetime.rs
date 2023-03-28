/*
 * Lifetime means the time between the allocation and automattic de allocation
 */

struct User<'x, T> {
    username: &'x str,
    password: &'x str,
    email_id: &'x str,
    favorite: &'x Vec<T>,
}

fn main(){

    let new_user: User<i32> = User {
        username: "KunalSin9h",
        password: "pass",
        email_id: "email@domain.com",
        favorite: &vec![1, 2, 3],
    };

    print_user(new_user);

    let again_new_user: User<char> = User{
        username: "New User",
        password: "New password",
        email_id: "em",
        favorite: &vec!['a', 'b', 'c'],
    };

    print_user(again_new_user);
}

fn print_user<T: std::fmt::Display>(user: User<T>) {
    println!("{}", user.username);
    println!("{}", user.password);
    println!("{}", user.email_id);

    for x in user.favorite.iter() {
        println!("{}", x);
    }
}
