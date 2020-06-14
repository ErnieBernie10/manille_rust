use uuid::Uuid;
use crate::deck::Hand;
use crate::card::Card;

pub struct Player {
    id: Uuid,
    hand: Hand,
    visible: Hand,
    invisible: Hand
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: Uuid::new_v4(),
            hand: Hand::new(),
            visible: Hand::new(),
            invisible: Hand::new()
        }
    }

    pub fn equals(&self, p: &Player) -> bool {
        self.id == p.id
    }

    pub fn give_card(&mut self, card: Card) {
        if self.invisible.len() < 4 {
            self.invisible.add_card(card);
        } else if self.hand.len() < 8 {
            self.hand.add_card(card);
        } else {
            self.visible.add_card(card);
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}{}", self.hand.to_string(), self.visible.to_string())
    }
}
