use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);


    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    let team_name2 = String::from("Yellow");

    scores.insert(&team_name, 10);
    scores.insert(&team_name2, 50);


    let score = scores.get(&team_name);

    println!("{} {:?}", team_name, score);
}
