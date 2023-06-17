const HAND_SIZE: usize = 5;

use rand::seq::SliceRandom;
use rand::thread_rng;

const SUITS: [&str;5] = ["Red", "Orange", "Yellow", "Green", "Blue"];

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

    pub fn missing_information_as_bits(&self) -> f32 {
        self.cards.iter()
            .map(|card| card.missing_information_as_bits())
            .sum() 
    }
}

pub struct Deck {
    deck: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck{
        let _suits:Vec<Suit> = vec!(Suit::Red, Suit::Yellow, Suit::White, Suit:: Blue, Suit::Green);
        let _numbers = vec!(Number::One, Number::Two, Number::Three, Number::Four, Number::Five);
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

    pub fn missing_information_as_bits(&self) -> f32 {
        let mut state_counts = 0;
        let mut options = self.options;
        while options > 0 {
            if options & 1 == 1 {
                state_counts += 1;
            }
            options = options >> 1;
        }
        return f32::log2(state_counts as f32);
    }
    

    pub fn remove_option(mut self, value: &CardValue) -> Self {
        let (number, suit) = value;
            self.options &= !((Card::ONE << number) & (Card::RED << (5 * suit)));
        self
    }

    pub fn remove_number(mut self, number: u8) -> Self {
        if number < 5 {
            self.options &= !(Card::ONE << number); 
        } else {
            panic!()
        }
        self
    }
    pub fn remove_suit(mut self, suit: u8) -> Self {
        if suit <= 4 {
            self.options &= !(Card::RED << (5 * suit))
        } else {
            panic!()
        }
        self
    }

    pub fn is_value(self, value: CardValue) -> Self {
        self.is_suit(value.1).is_number(value.0)
    }

    pub fn is_suit(mut self, suit: u8) -> Self {
        if suit <= 4 {
            self.options &= Card::RED << (5 * suit)
        }
        self
    }

    pub fn is_number(mut self, number: u8) -> Self {
        if number < 5 {
            self.options &= Card::ONE << number;
        }
        self
    }

    pub fn learn_suit(self, suit: u8) -> Self {
        if self.value.1 == suit {
            self.is_suit(suit)
        } else {
            self.remove_suit(suit)
        }
    }
    
    pub fn learn_number(self, number: u8) -> Self {
        if self.value.0 == number {
            self.is_number(number)
        } else {
            self.remove_number(number)
        }
    }
    pub fn print(&self) {
        let (num, suit) = self.value;
        print!("\n{:?} of {}\n", num, SUITS[suit as usize]);
        Self::print_option(self.options);
    }

    pub fn print_option(options: u32) {   
        for n in 0..5 {
            for s in 0..5 {
                if (options >> (n + (s * 5))) & 1 == 1 {
                    print!("██");
                } else {
                    print!("░░");
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

    pub fn as_u8(&self) -> u8 {
        use Suit::*;
        match self {
            Red => 1,
            Blue => 2,
            White => 3,
            Green => 4,
            Yellow => 5
        }
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
        use Number::*;
        return [
            One,
            Two,
            Three,
            Four,
            Five
        ]
    }

    pub fn as_u8(&self) -> u8 {
        use Number::*;
        return match self {
            One => 1,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5 // Wow such counting
        }
    }
}

#[cfg(test)]

mod test{
    use super::*;
    #[test]
    fn total_information(){
        let mut card = Card::new((2,3));
        // total bits of inforamtion of a card is ~4.6 bits
        assert_eq!((card.missing_information_as_bits() * 10.0).round(), 46.0);
        card = card.is_value((2,3));
        assert_eq!((card.missing_information_as_bits() * 10.0).round(), 0.0);
    }
}
