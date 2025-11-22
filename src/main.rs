mod types;

fn main() {
    let options = vec![
        "Gandu is correct".to_string(),
        "Nigger is correct".to_string(),
        "Soover is correct".to_string(),
        "Bharathesh is correct".to_string(),
    ];
    let hello = vec![types::Question {
        question: "Hi Hello how are you".to_string(),
        options: options,
        correct: 3,
        selected: None,
    }];
    let mut mcq = types::Mcq::new(hello);
    mcq.print();
    println!("{:?}", mcq);
}
