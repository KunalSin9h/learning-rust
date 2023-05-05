use std::fs;

fn main(){

    let file = fs::read_to_string("text_file.txt").unwrap();

    file
        .lines() // return an iterator, split at \n on a string
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|tpl| println!("{}", tpl.1));
}
