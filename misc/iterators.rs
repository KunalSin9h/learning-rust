use std::collections::{HashSet, HashMap};

fn main(){
    let data = vec![1, 2, 3];
    let mut list = data
        .iter() // convert collection into iterator
        .map(|x| x + 1); // iterate over the iterator

    let mut new_list = vec![];

    while let Some(x) = list.next() {
        new_list.push(x);
    }
                    
    println!("{:?}", new_list);

    let sentence: String = vec!["hello", "i", "am", "kunal"]
        .into_iter()
        .collect();

    println!("{}",sentence);

    let data = vec![1, 2, 3, 3, 2, 1, 1, 1, 1];

    let unique: HashSet<i32> = data.into_iter()
        .collect();
    dbg!(unique);

    let freq: HashMap<i32, usize> = vec![1, 2, 2, 1, 1, 1, 2, 3]
        .into_iter()
        .enumerate() // add index to iterator
        .map(|(idx, item)| (item, idx))
        .collect();

    dbg!(freq);
}
