const HAND_SIZE: usize = 5;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy)]
pub struct Hand {
    pub cards : [Card; HAND_SIZE] 
}

impl Hand {
    pub const fn new() -> Hand {
        Hand {
            cards: [Card::new((0, 0)); HAND_SIZE]
        }
    }

    pub fn print(&self) {
        for i in 0..HAND_SIZE {
            println!("{}", i);
            self.cards[i].print();
        }
    }
}

pub struct Deck {
    deck: Vec<Card>
}

impl Deck {

    pub fn new() -> Deck{
        let suits:Vec<Suit> = vec!(Suit::Red, Suit::Yellow, Suit::White, Suit:: Blue, Suit::Green);
        let numbers = vec!(Number::One, Number::Two, Number::Three, Number::Four, Number::Five);
        let mut result = Vec::new();
        for number in 0..4 {
            for suit in 0..4 {
                let value = (number, suit);
                result.push(Card::new(value));
            }
        }
        return Deck{deck:result};
    }

    pub fn shuffle(mut self) -> Deck {
        self.deck.shuffle(&mut thread_rng());
        self
    }

    pub fn pull(&mut self) -> Option<Card> {
        return self.deck.pop()
    }
}

type CardValue = (u8, u8);
#[derive(Copy, Clone, Debug)]
pub struct Card {
    options: u32,
    value: CardValue,
}

impl Card {
    const RED:u32 = (1 << 5) - 1;
    const ONE:u32 = 1 | 1 << 5 | 1 << 10 | 1 << 15 | 1 << 20;

    pub const fn new(value: CardValue) -> Card {
        Card {
            options: (1 << 25) - 1,
            value: (value)
        }
    }

    pub fn value(&self) -> &CardValue {
        &self.value
    }


    pub fn remove_option(&mut self, value: &CardValue) {
        let (number, suit) = value;
        self.options &= !((Card::ONE << number) & (Card::RED << (5 * suit)));
    }

    pub fn remove_number(&mut self, number: u8) {
        if number < 5 {
            self.options &= !(Card::ONE << number); 
        } else {
            panic!()
        }
    }
    pub fn remove_suit(&mut self, suit: u8) {
        if suit <= 4 {
            self.options &= !(Card::RED << (5 * suit))
        } else {
            panic!()
        }
    }

    pub fn is_suit(&mut self, suit: u8) {
        if suit <= 4 {
            self.options &= Card::RED << (5 * suit)
        }
    }

    pub fn is_number(&mut self, number: u8) {
        if number < 5 {
            self.options &= Card::ONE << number;
        }
    }
    
    pub fn print(&self) {
        let (num, suit) = self.value;
        print!("\n{:?} of {:?}\n", suit, num);
        Self::print_option(self.options);
    }

    pub fn print_option(options: u32) {   
        for n in 0..5 {
            for s in 0..5 {
                if (options >> (n + (s * 5))) & 1 == 1 {
                    print!("██");
                } else {
                    print!("▏▕");
                }
            }
            print!("\n");
        }
    }


}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Suit {
    Red,
    Blue,
    White,
    Green,
    Yellow
}
    
impl Suit {
    fn all() -> [Suit; 5]{
        return [ 
            Suit::Red,
            Suit::Blue,
            Suit::White,
            Suit::Green,
            Suit::Yellow
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five
}

impl Number {
    fn all() -> [Number; 5] {
        return [
            Number::One,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five
        ]
    }
}
