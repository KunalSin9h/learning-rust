trait Fly {
    async fn can_fly() -> bool;
}

async fn hello() {
    return "Hello";
}

fn main(){
    let x = hello();
    let x = hello();
}
