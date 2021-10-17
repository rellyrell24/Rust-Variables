const MAX_POINTS: u32 = 100_000;

fn main() {
    // Mutability
    println!("The value of Max Points is: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len();  mismatches types error

    // Floating points
    let x = 2.0; // Default: f64
    let y: f32 = 3.0;

    // Numeric Operations
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    // Boolean Type
    let t = true;
    let f: bool = false; // explicitly type annotation

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; // b = [3,3,3,3,3]

    let first = a[0];
    let second = a[1];

    // Functions
    another_function(first, second);

    // Statements and Expressions
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    // Functions with return values
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    // Control Flow
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // arms must evaluate to same type
    println!("The value of number is: {}", number);

    // Repetition with loops
    // loop, while, for
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // returns last expression implicitly
}

fn plus_one(x: i32) -> i32 {
    x + 1 // semicolon converts to statement
}