use std::collections::HashSet;
#[allow(unused)]
// #[allow(dead_code)]
// #[allow(unused_variables)]

fn main() {
    let mut numbers: Vec<i32> = vec![];
    let words = vec!["step", "on", "no", "pets"];
    let mut buffer = vec![0u8; 1024];

    let my_set = ["one", "two", "three", "four", "five", "six"];

    let my_vec = my_set.into_iter().collect::<Vec<&str>>();
   
    let line = [0, 1, 2, 4, 5, 6];

    // Получить ссылку на элемент
    let first_line = &line[0];

    // Получить копию элемента
    let fifth_number = line[0];       // требуется трейт Copy
    let second_line = line[1].clone();  // требуется трейт Clone

    // Получить ссылку на slice
    let my_ref = &buffer[4..12];

    // Получить копию slice
    let my_copy = buffer[4..12].to_vec(); // требуется трейт Clone

    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(v.to_vec(),
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(v[0..6].to_vec(),
                vec!{1, 2, 3, 4, 5, 6});


    let mut byte_vec = b"Missssssssisssippi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    let mut byte_vec = b"Missssssssisssippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");

    println!("{:?}", seen);

    //Reading Elements of Vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    // Iterating over mutable references to elements in a vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);


    // Using an Enum to Store Multiple Types
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


    // Creating a hash map from a list of teams and a list of scores
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();



    // Showing that keys and values are owned by the hash map once they’re inserted
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!




    // Only Inserting a Value If the Key Has No Value
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);  // {"Blue": 10, "Yellow": 50}




    // Counting occurrences of words using a hash map that 
    // stores words and counts
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
