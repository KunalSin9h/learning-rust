struct Point {
    x: i64,
    y: i64,
    z: i64,
} // Point

fn main(){

    let ball_point = Point {
        x: 1, 
        y: 2, 
        z: 3,
    };

    let Point {z, y, ..} = ball_point; // named, not by position

    let sum = ball_point.x + ball_point.y + ball_point.z;

    println!("{}, {}", y,  z);
    println!("The sum of coordinates is {}", sum);


    let mut mutable_point = Point {
        x: 0,
        y: 0,
        z: 0,
    };
    
    mutable_point.x = 10;
    mutable_point.y = 11;
    mutable_point.z = 12;

    println!("{}, {}, {}", mutable_point.x, mutable_point.z, mutable_point.y);

    let mut not_init_point: Point; // declaration
    not_init_point = Point{x: 0, y: 0, z: 0}; // initialization
                                              //
    /*
     * WE SHOULD INITIALIZE BEFORE MUTATE / USE
     */

    not_init_point.x = 45; // | 
    not_init_point.y = 67; // |   mutation
    not_init_point.z = 99; // | 
    println!("{}, {}, {}", not_init_point.x, not_init_point.y, not_init_point.z); // use

}
