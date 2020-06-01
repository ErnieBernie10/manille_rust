pub struct Card {
    pub suit: String,
    pub rank: String,
    pub points: String
}

impl Card {
    pub fn new(suit: String, rank: String) -> Card {
        Card {
            suit: suit,
            rank: rank,
            points: "".to_string()
        }
    }
}