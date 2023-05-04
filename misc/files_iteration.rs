use std::fs;

fn main(){

    let file = fs::read_to_string("text_file.txt").unwrap();

    file
        .lines() // return an iterator, split at \n on a string
        .enumerate()
        .filter(|(idx, _)| {
            if idx % 2 == 0 {
                return true;
            }
            return false;
        })
        .for_each(|tpl| println!("{}", tpl.1));

}
