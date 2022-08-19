fn main() {
    // ----- Rules of References -----
    // - At any given time, you can have either one mutable reference or any number of immutable references.
    // - References must always be valid.

    let string_to_find_length = String::from("hello");
    let length = calculate_length(&string_to_find_length);

    println!("The length of {} is {}.", string_to_find_length, length);

    let mut string_to_mutate = String::from("hello");
    can_change_variable(&mut string_to_mutate);
    println!("The new value is {}", string_to_mutate);

    // ----- Slices -----
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    using_slices();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn can_change_variable(v: &mut String) {
    v.push_str(" again");
    println!("The value post change is {}", v);
}
fn using_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("First word is {}", hello);
    println!("Second word is {}", world);

    // these two are the same, you can omit the starting 0
    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];

    // so are these two for th end of the string
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // you can drop both to take a slice of the full string
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];
}
