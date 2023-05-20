trait Animal {
   fn talk(&self);
}

struct Cat {} 
struct Dog {}

impl Animal for Cat {
  fn talk(&self) {
    println!("meow");
  }
}

impl Animal for Dog {
  fn talk(&self) {
    println!("bark");
  }
}

fn animal_talk(a: &dyn Animal) { // dynamic 
                                 // so, it means not known at compile time
                                 // hence we need a pointer to make it dynamic
                                 // hence &
  a.talk();
}

fn main() {
  let d = Dog {};
  let c = Cat {};

  animal_talk(&d);
  animal_talk(&c);
}
