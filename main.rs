
fn main() {
    // Using 'mut' allows you to mutate the x variable.
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // for 'const' you need to define the variable type;
    // const MAX_POINTS: u32 = 100_100;
    // println!("The value of max is: {}", MAX_POINTS);

    // Shadowing
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is: {}", x);

    // More shadowing however, if I was to use 'mut' it will bring out an error saying that you cannot
    // mutate this variable as it is a string.
    // let spaces = "     ";

    // let spaces = spaces.len();

    // println!("This is spaces, {}", spaces);


    // Tupe variables

    // let x: (i32, f64, u8) = (500, 42.5, 4);

    // let five_hundred = x.0;

    // let fourty_two_point_five = x.1;

    // let four = x.2;

    // println!("500 is, {}", five_hundred);

    // Using if statements in let variable

    let condition = false;
    // We cannot however have a conditional output of a string or a integer.
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}