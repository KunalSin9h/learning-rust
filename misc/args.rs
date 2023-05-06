use std::env;
use std::fs;

fn main(){
     let file_name = env::args().nth(1)
         .expect("File name not specified");

     let file_content = fs::read_to_string(&file_name)
         .expect("File does not exist");

    file_content
        .lines()
        .for_each(|line| {
            if let Ok(value) = line.parse::<usize>() {
                println!("{}", value);
            } else {
                println!("Line not a number");
            }
        });
}
