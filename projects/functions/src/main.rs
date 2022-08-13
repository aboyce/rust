fn main() {
    println!("functions");

    // Statements are instructions that perform some action and do not return a value.
    expression();

    // Expressions evaluate to a resulting value.
    let _five = function_with_return_value();
}

fn expression() {
    // A new scope block is an expression
    let y = {
        let x = 3;
        x + 1 // expressions do not have an semi-colon, if you add one, it will become a statement and not an expression
    };

    println!("The value of y is: {y}");
}

fn function_with_return_value() -> u32 {
    5
}
