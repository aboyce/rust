fn main() {
    println!("Hello, world!");
    vectors();
    strings();
    hash_maps();
}

fn vectors() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(10);
    v.push(15);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let fourth = v.get(3);

    fourth.expect("A value");

    match fourth {
        Some(fourth) => println!("The fourth element is {}", fourth),
        None => print!("Nowt"),
    }

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterating over values and can mutate
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // being able to store various types in a vector using an enum (they can only store the same type)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn strings() {
    // Strings behave a lot like collections as they are build on top of the Vector (Vec<u8>) with additional helpers

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // growing a string
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push takes a single character
    let mut s = String::from("lo");
    s.push('l');

    // concatenation with the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // the + operator can get a bit unwieldy
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // can be easier to use the format macro (the format macro works like println but returns the string rather than prints it)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format also uses references so does not take ownership of anything
    let s = format!("{}-{}-{}", s1, s2, s3);

    // the safest way to iterate over a string
    for c in "Зд".chars() {
        println!("{}", c);
    }
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

fn hash_maps() {
    //  Many programming languages support this kind of data structure, but they often use a different name,
    // such as hash, map, object, hash table, dictionary, or associative array, just to name a few.

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other,
    // and all of the values must have the same type.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // can iterate over the key/values
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and

    // overwriting the value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // adding a key if it is not already present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // updating a value based on the existing value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
