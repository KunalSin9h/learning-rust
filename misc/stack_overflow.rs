#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_items(items: &Vec<Item>) {
    // items
    //     .iter()
    //     .for_each(|item| println!("{:?}", item));

    for item in items {
        println!("{:?}", item);
    }
}

fn main(){

    let mut item = Item { count: 0 };
    add_one(&mut item);

    let item2 = Item { count: 0 };

    let mut items: Vec<Item> = vec![item, item2];

    let first = items.first_mut(); // it has a mutable ref
    let second = items.get_mut(1); // there can not be 2 mut ref

    println!("{:?}", second);
    print_items(&items); // so how this thing has immut ref ?

    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // add_one(&mut item);
    // dbg!(item);

}
