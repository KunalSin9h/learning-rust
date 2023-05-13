fn main(){
    let x = 10;
    // let y: *mut i32 = &mut x;
    // unsafe {
    //     println!("{:?}", y);
    // }

    let a: *const i32 = &x;

    unsafe {
        println!("{}", *a);
    }
}
