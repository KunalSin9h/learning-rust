fn return_number(x: Option<usize>) -> Option<usize> {
    let x = x?; // if x is None then None will be return from this line
    return Some(x * 5);
    /*
     * applying Map on Option
     */
    // return x.map(|y| y * 5);


    // match x {
    //     None => return None, 
    //     Some(y) => return Some(y * 5),
    // }
}

fn practice(nums: Vec<usize>, idx: usize) -> usize {
    return nums.get(idx).unwrap_or(&idx) * 5;
}

fn main(){
    dbg!(return_number(Some(10)));
    dbg!(practice(vec![1, 2, 3], 0));
    dbg!(practice(vec![1, 2, 3], 1));
    dbg!(practice(vec![1, 2, 3], 2));
    dbg!(practice(vec![1, 2, 3], 3));
}
