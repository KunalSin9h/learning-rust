struct TypeWithDestructor(i32);

impl Drop for TypeWithDestructor {
    fn drop(&mut self) {
        println!("Dropped. Held {}.", self.0);
    }
}

const ZERO_WITH_DESTRUCTOR: TypeWithDestructor = TypeWithDestructor(10);

fn create_and_drop_zero_with_destructor() {
    let _ = ZERO_WITH_DESTRUCTOR;
    // x gets dropped at end of function, calling drop.
    // prints "Dropped. Held 0.".
}

fn main(){
    create_and_drop_zero_with_destructor(); 

    let _ = TypeWithDestructor(100);

}
