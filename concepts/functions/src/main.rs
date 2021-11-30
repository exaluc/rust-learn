fn main() {
    println!("Hello, world!");

    // Function
    another_function();

    // Function Parameters
    print_labeled_measurement(5, 'h');
    
    another_function2(5);

    // Function Bodies Contain Statements and Expressions
    let _x = 5;
    let y = {
        let _x = 3;
        _x + 1
    };
    println!("The value of y is: {}", y);

    // Functions with Return Values
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);



}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}