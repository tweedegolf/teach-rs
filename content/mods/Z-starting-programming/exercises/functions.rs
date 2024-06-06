fn x_squared_plus_one(number: i32) -> i32 {
    // TODO: Implement the expression for this
}

fn double(number: i32) -> i32 {
    2 * number
}

fn main() {
    let double_6 = double(6);
    println!("2 * 6: {double_6}");

    let six_squared_plus_one = x_squared_plus_one(6);
    println!("6*6 + 1: {six_squared_plus_one}");

    // TODO: Create the function named `x_cubed_plus_x_squared`
    let six_cubed_plus_six_squared = x_cubed_plus_x_squared(6);
    println!("6*6*6 + 6*6: {six_cubed_plus_six_squared}");
}