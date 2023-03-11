const HAND_SIZE:usize = 5;
mod Card;
use std::num;

use Card::Deck;

fn main() {
    println!("Hello, world!");
    let mut deck = Deck::new().shuffle();
    let mut my_hand: [Card::Card; 5] = [Card::Card::new((0, 0)); 5];
    let mut your_hand: [Card::Card; 5] = [Card::Card::new((0, 0)); 5];
    for i in 0..5 {
        print!("Pulling card {}", i);
        my_hand[i] = deck.pull().unwrap();
        your_hand[i] = deck.pull().unwrap();
}
    for mut card in your_hand {
        card.is_number(card.value().0.clone());
        card.is_suit(card.value().1.clone());
        for i in 0..5 {
            let mut my_card = my_hand[i];
            my_card.remove_option(card.value());
            my_hand[i] = my_card
        }
        print!("--\n");
        card.print();
    }

    let (number1, suit1) = my_hand[0].value().clone();
    let (number2, suit2) = my_hand[1].value().clone();

    for i in 0..5 {
        let mut my_card = my_hand[i];
        let (card_number, card_value) = my_card.value().clone();
        if number1 == card_number {
            my_card.is_number(number1)
        } else {
            my_card.remove_number(number1)
        }

        if suit2 == card_value {
            my_card.is_suit(suit2)
        } else {
            my_card.remove_suit(suit2)
        }
        my_hand[i] = my_card
    }


    for my_card in my_hand {
        my_card.print();
    }

}

struct Game {
    hands: [Card::Hand; HAND_SIZE],
    middle: [u8; 5],
    turn: usize,
    resource: u8,
    fuse: u8,
    deck: Card::Deck,
}

impl Game {
    pub fn new() -> Game {
        // create deck
        // Shuffle 
        // deal
        Game {
            hands: [Card::Hand::new(); HAND_SIZE],
            middle: [0;5],
            turn: 0,
            resource: 8,
            fuse: 2,
            deck: Card::Deck::new(),
        }
    }


    pub fn play(action:Action) {
       match action {
           Action::Play(u8) => {
           },
           Action::Discard(u8) => {
           },
           Action::GiveInformation(info) => {
           }
       }
    }

    pub fn is_won(&self) -> bool {
        self.middle == [5;5]
    }


}

enum Action {
    Play(usize),
    Discard(usize),
    GiveInformation(InformationType)
}

enum InformationType {
    Colour(u8),
    Number(u8)
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn setup() {
        let game = Game();
    }

}
