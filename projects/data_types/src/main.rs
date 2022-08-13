fn main() {
    println!("data_types");

    // Scalar Types
    let _eight_bit: u8 = 1;
    let _sixteen_bit: u16 = 1;
    let _thirty_two_bit: u32 = 1;
    let _sixty_four_bit: u64 = 1;
    let _one_hundred_twenty_eight_bit: u128 = 1;
    let _architecture_bit: usize = 1;

    let _decimal: u8 = 1;
    let _hex: u8 = 0xff;
    let _octal: u8 = 0o77;
    let _binary: u8 = 0b1111_0000;
    let _byte: u8 = b'A';

    let _default_floating: f64 = 1.23;
    let _thirty_two_bit_floating: f32 = 1.23;

    let _boolean_true: bool = true;
    let _boolean_false: bool = false;

    let _character: char = 'A';
    let _emoji_support: char = '🥹';

    // Compound Types

    // * A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    //  * Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let _tuple: (u32, f64, u8) = (500, 6.4, 1);
    // You can destructure a tuple
    let (_a, _b, _c) = _tuple;
    // You can access parts of a tuple with dot notation
    let _d = _tuple.0;
    let _e = _tuple.1;
    let _f = _tuple.2;

    // Arrays are useful when you want your data allocated on the stack rather than the heap
    let _array: [char; 3] = ['A', 'B', 'C'];
    // You can create an array with the same value
    let _same_array = [5; 5];
}
