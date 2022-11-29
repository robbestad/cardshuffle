use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};

fn main() {
    //           diamonds    clubs       hearts      spades
    let col = ["\u{2666}", "\u{2663}", "\u{2665}", "\u{2660}"];
    let val = [
        "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "A", "K", "Q", "J", "T", "9",
        "8", "7", "6", "5", "4", "3", "2", "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4",
        "3", "2", "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "A", "K", "Q",
        "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "A", "K", "Q", "J", "T", "9", "8", "7",
        "6", "5", "4", "3", "2", "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
        "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2", "A",
    ];

    let mut doc: Vec<String> = col
        .iter()
        .flat_map(|c| val.iter().clone().map(move |v| v.to_string() + c))
        .collect();

    let data: String = std::fs::read_to_string("random.txt")
        .unwrap()
        .to_owned()
        .replace("\n", "");
    let seed: [u8; 32] = data.as_bytes().try_into().unwrap();
    println!("seed {}", data);
    let mut seeded_rng = StdRng::from_seed(seed);
    doc.shuffle(&mut seeded_rng);
    println!("{} deck shuffle:\n{:?}\n", doc.len() / 52, doc);
}
