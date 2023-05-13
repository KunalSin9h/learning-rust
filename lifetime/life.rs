// /*
// the case of invalid borrow
//  */
// fn main() {
//     let r: &u32; // borrower

//     {
//         let x: u32 = 10; // lender
//         r = &x;
//     } // lender dies here
//       // i.e lender dies before the borrower -> Borrow is invalid

//     println!("{}", r);
// }

fn main() {
    let a = String::from("AB");
    let res: String; // blank
    {
        let b = String::from("B");

        res = longest(&a, &b);
    }
    println!("{}", res);
}
/*
&'a String in return is the has the lifetime of the smallest lifetime
among the parameters */

fn longest<'a>(x: &String, y: &String) -> String {
    let p = String::from("P");
    return p;
    // if x.len() > y.len() {
    //     return x;
    // } else {
    //     return y;
    // }
}
