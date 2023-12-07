use std::cmp::{min, Ordering};
use AoC2023::InputType;
use std::collections::HashMap;
use AoC2023::string_to_u32;
use itertools::Itertools;

pub fn day07(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 7");


    let data = AoC2023::read_input(7, input_type, manual_name)?;

    // --- Day 7: Camel Cards ---
    // Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

    // "Did you bring the parts?"

    // You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

    // "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

    // "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

    // After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

    // You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

    // Because the journey will take a few days, she offers to teach you the game of Camel Cards.
    // Camel Cards is sort of similar to poker except it's designed to be easier to play while
    // riding a camel.

    // In Camel Cards, you get a list of hands, and your goal is to order them based on the strength
    // of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3,
    // or 2. The relative strength of each card follows this order, where A is the highest and 2
    // is the lowest.

    // Every hand is exactly one type. From strongest to weakest, they are:

    // Five of a kind, where all five cards have the same label: AAAAA
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // Full house, where three cards have the same label, and the remaining two cards share a
    // different label: 23332
    // Three of a kind, where three cards have the same label, and the remaining two cards are
    // each different from any other card in the hand: TTT98
    // Two pair, where two cards share one label, two other cards share a second label, and the
    // remaining card has a third label: 23432
    // One pair, where two cards share one label, and the other three cards have a different
    // label from the pair and each other: A23A4
    // High card, where all cards' labels are distinct: 23456
    // Hands are primarily ordered based on type; for example, every full house is stronger
    // than any three of a kind.

    // If two hands have the same type, a second ordering rule takes effect. Start by comparing
    // the first card in each hand. If these cards are different, the hand with the stronger
    // first card is considered stronger. If the first card in each hand have the same label,
    // however, then move on to considering the second card in each hand. If they differ, the
    // hand with the higher second card wins; otherwise, continue with the third card in each
    // hand, then the fourth, then the fifth.

    // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its
    // first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888
    // is stronger because its third card is stronger (and both hands have the same first
    // and second card).

    // To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483
    // This example shows five hands; each hand is followed by its bid amount.
    // Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand
    // gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand.
    // Because there are five hands in this example, the strongest hand will have rank 5 and
    // its bid will be multiplied by 5.

    // So, the first step is to put the hands in order of strength:

    // 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    // KK677 and KTJJT are both two pair. Their first cards both have the same label, but the
    // second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    // T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets
    // rank 5 and T55J5 gets rank 4.
    // Now, you can determine the total winnings of this set of hands by adding up the result
    // of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5).
    // So the total winnings in this example are 6440.

    // Find the rank of every hand in your set. What are the total winnings?

    // for row in &data {
    //     println!("{:?}", row);
    // }

    #[derive(Eq)]
    struct Hand {
        cards : Vec<char>,
        bid : u32,
    }
    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards == other.cards
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let type1 = get_hand_type(&self.cards);
            let type2 = get_hand_type(&other.cards);
            let mut order : Ordering = Ordering::Equal;

            if type1 > type2 {
                order= Ordering::Greater;
            }
            else if type2 > type1 {
                order= Ordering::Less;
            }
            else {
                for i in 0..min(self.cards.len(), other.cards.len()) {
                    if card_value_to_type(&self.cards[i]) > card_value_to_type(&other.cards[i])
                    {
                        order = Ordering::Greater;
                        break;
                    }
                    else if card_value_to_type(&self.cards[i]) < card_value_to_type(&other.cards[i])
                    {
                        order = Ordering::Less;
                        break;
                    }
                }
            }
            order
        }

    }

    // cards with relative values, again for comparison
    #[derive(PartialEq, PartialOrd)]
    enum Cards {
        A = 14,
        K = 13,
        Q = 12,
        J = 11,
        T = 10,
        Nine = 9,
        Eight = 8,
        Seven = 7,
        Six = 6,
        Five = 5,
        Four = 4,
        Three = 3,
        Two = 2,

    }

    // types of hands with values for determining who wins
    #[derive(Debug, PartialEq, PartialOrd)]
    enum HandTypes {
        Five = 7,
        Four = 6,
        Full = 5,
        Three = 4,
        Two = 3,
        One = 2,
        High = 1,
    }

    fn get_hand_type (hand: &Vec<char>) -> HandTypes {

        // count the number of equal elements per hand
        let mut map = HashMap::new();

        for item in hand {
            *map.entry(item).or_insert(0) += 1;
        };

        let mut hand_type: HandTypes = HandTypes::High;
        // all equal
        match map.len() {
            1 => { hand_type = HandTypes::Five; }
            2 => {
                if map.values().any(|&x| x == 4 as u32) {
                    hand_type = HandTypes::Four;
                } else { hand_type = HandTypes::Full; }
            }
            3 => if map.values().any(|&x| x == 3 as u32) {
                hand_type = HandTypes::Three;
            } else if map.values().any(|&x| x == 2 as u32) {
                hand_type = HandTypes::Two;
            }
            4 => { hand_type = HandTypes::One; }
            _ => { hand_type = HandTypes::High; }
        }
        hand_type
    }

    fn card_value_to_type (card: &char) -> Cards {
        match card {
            'A' => Cards::A,
            'K' => Cards::K,
            'Q' => Cards::Q,
            'J' => Cards::J,
            'T' => Cards::T,
            '9' => Cards::Nine,
            '8' => Cards::Eight,
            '7' => Cards::Seven,
            '6' => Cards::Six,
            '5' => Cards::Five,
            '4' => Cards::Four,
            '3' => Cards::Three,
            _   => Cards::Two,
        }
    }


    // process data
    let mut full_set : Vec<Hand> = Vec::new();
    for row in &data {
        let hand: Hand = Hand {cards: row[0].chars().collect(), bid:string_to_u32(&row[1])};
        // println!("{:?}", &hand.cards);
        // println!("{:?}",get_hand_type(&hand.cards));

        // println!("{}", &hand.bid);
        full_set.push(hand);
    }

    let mut total_score : u32 = 0;
    let mut id : u32 = 1;
    for hand in full_set.iter().sorted() {
        // println!("{:?}", &hand.cards);
        // println!("rank: {}, bid: {}", id, hand.bid);
        total_score += id * hand.bid;
        id += 1;

    }
    println!("Part 1: {total_score}");
    // 250602641 is correct


    Ok(())
}

pub fn day72(input_type: InputType, manual_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Day 7 - part 2");


    let data = AoC2023::read_input(7, input_type, manual_name)?;

    // --- Day 7: Camel Cards ---
    // Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.

    // "Did you bring the parts?"

    // You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.

    // "Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.

    // "The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.

    // After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon. The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there. Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.

    // You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.

    // Because the journey will take a few days, she offers to teach you the game of Camel Cards.
    // Camel Cards is sort of similar to poker except it's designed to be easier to play while
    // riding a camel.

    // In Camel Cards, you get a list of hands, and your goal is to order them based on the strength
    // of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3,
    // or 2. The relative strength of each card follows this order, where A is the highest and 2
    // is the lowest.

    // Every hand is exactly one type. From strongest to weakest, they are:

    // Five of a kind, where all five cards have the same label: AAAAA
    // Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // Full house, where three cards have the same label, and the remaining two cards share a
    // different label: 23332
    // Three of a kind, where three cards have the same label, and the remaining two cards are
    // each different from any other card in the hand: TTT98
    // Two pair, where two cards share one label, two other cards share a second label, and the
    // remaining card has a third label: 23432
    // One pair, where two cards share one label, and the other three cards have a different
    // label from the pair and each other: A23A4
    // High card, where all cards' labels are distinct: 23456
    // Hands are primarily ordered based on type; for example, every full house is stronger
    // than any three of a kind.

    // If two hands have the same type, a second ordering rule takes effect. Start by comparing
    // the first card in each hand. If these cards are different, the hand with the stronger
    // first card is considered stronger. If the first card in each hand have the same label,
    // however, then move on to considering the second card in each hand. If they differ, the
    // hand with the higher second card wins; otherwise, continue with the third card in each
    // hand, then the fourth, then the fifth.

    // So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its
    // first card is stronger. Similarly, 77888 and 77788 are both a full house, but 77888
    // is stronger because its third card is stronger (and both hands have the same first
    // and second card).

    // To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:

    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483
    // This example shows five hands; each hand is followed by its bid amount.
    // Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand
    // gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand.
    // Because there are five hands in this example, the strongest hand will have rank 5 and
    // its bid will be multiplied by 5.

    // So, the first step is to put the hands in order of strength:

    // 32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
    // KK677 and KTJJT are both two pair. Their first cards both have the same label, but the
    // second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
    // T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets
    // rank 5 and T55J5 gets rank 4.
    // Now, you can determine the total winnings of this set of hands by adding up the result
    // of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5).
    // So the total winnings in this example are 6440.

    // Find the rank of every hand in your set. What are the total winnings?

    // for row in &data {
    //     println!("{:?}", row);
    // }


    // --- Part Two ---
    // To make things a little more interesting, the Elf introduces one additional rule. Now, J
    // cards are jokers - wildcards that can act like whatever card would make the hand the
    // strongest type possible.
    //
    // To balance this, J cards are now the weakest individual cards, weaker even than 2.
    // The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
    //
    // J cards can pretend to be whatever card is best for the purpose of determining hand type;
    // for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking
    // ties between two hands of the same type, J is always treated as J, not the card it's
    // pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
    //
    // Now, the above example goes very differently:
    //
    // 32T3K 765
    // T55J5 684
    // KK677 28
    // KTJJT 220
    // QQQJA 483
    // 32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't
    // increase.
    // KK677 is now the only two pair, making it the second-weakest hand.
    // T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4,
    // and KTJJT gets rank 5.
    // With the new joker rule, the total winnings in this example are 5905.
    //
    // Using the new joker rule, find the rank of every hand in your set. What are the new total
    // winnings?

    #[derive(Eq)]
    struct Hand {
        original: String,
        cards : Vec<char>,
        joker_cards : Vec<char>,
        bid : u32,
    }
    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.cards == other.cards
        }
    }
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            // hand type check done with jokers
            let type1 = get_hand_type(&self.joker_cards);
            let type2 = get_hand_type(&other.joker_cards);
            let mut order : Ordering = Ordering::Equal;

            if type1 > type2 {
                order= Ordering::Greater;
            }
            else if type2 > type1 {
                order= Ordering::Less;
            }
            else {
                // card comparisons done with the normal cards
                for i in 0..min(self.cards.len(), other.cards.len()) {
                    if card_value_to_type(&self.cards[i]) > card_value_to_type(&other.cards[i])
                    {
                        order = Ordering::Greater;
                        break;
                    }
                    else if card_value_to_type(&self.cards[i]) < card_value_to_type(&other.cards[i])
                    {
                        order = Ordering::Less;
                        break;
                    }
                }
            }
            order
        }

    }

    // cards with relative values, again for comparison
    #[derive(PartialEq, PartialOrd)]
    enum Cards {
        A = 14,
        K = 13,
        Q = 12,
        // J = 11,
        T = 10,
        Nine = 9,
        Eight = 8,
        Seven = 7,
        Six = 6,
        Five = 5,
        Four = 4,
        Three = 3,
        Two = 2,
        J = 1, //FFS

    }

    // types of hands with values for determining who wins
    #[derive(Debug, PartialEq, PartialOrd)]
    enum HandTypes {
        Five = 7,
        Four = 6,
        Full = 5,
        Three = 4,
        Two = 3,
        One = 2,
        High = 1,
    }

    fn get_hand_type (hand: &Vec<char>) -> HandTypes {

        // count the number of equal elements per hand
        let mut map = HashMap::new();

        for item in hand {
            *map.entry(item).or_insert(0) += 1;
        };

        let mut hand_type: HandTypes = HandTypes::High;
        // all equal
        match map.len() {
            1 => { hand_type = HandTypes::Five; }
            2 => {
                if map.values().any(|&x| x == 4 as u32) {
                    hand_type = HandTypes::Four;
                } else { hand_type = HandTypes::Full; }
            }
            3 => if map.values().any(|&x| x == 3 as u32) {
                hand_type = HandTypes::Three;
            } else if map.values().any(|&x| x == 2 as u32) {
                hand_type = HandTypes::Two;
            }
            4 => { hand_type = HandTypes::One; }
            _ => { hand_type = HandTypes::High; }
        }
        hand_type
    }

    fn add_jokers(hand: &Vec<char>) -> Vec<char> {

        // count the number of equal elements per hand
        let mut map = HashMap::new();
        let mut joker_hand = Vec::new();

        for item in hand {
            *map.entry(item).or_insert(0) += 1;
        };

        if !map.keys().contains(&&'J') {
            joker_hand = hand.clone();
        }
        else {
            match map.len() {
                1 => { joker_hand = vec!['A';5]; } // all are jokers already
                2 => {
                    for key in map.keys() {
                        if key != &&'J' {
                            joker_hand = vec![**key, **key, **key, **key, **key];
                            break;
                        }
                    }
                }
                3 | 4 => {
                    // replace J with most frequent
                    let mut new_map = map.clone();
                    new_map.remove(&'J');
                    let high_val = new_map.values().max().unwrap();
                    let highest = new_map.iter()
                        .find_map(|(key, &val)| if &val == high_val { Some(key) } else { None }).unwrap();
                    for k in hand {
                        if k == &'J' {
                            joker_hand.push(*highest.clone());
                        }
                        else {joker_hand.push(*k)}
                    }
                }
               _ => {
                    // remove J from dict, sort, and replace Js with highest card
                    let mut new_map = map.clone();
                    new_map.remove(&'J');
                    let highest = new_map.keys().max().unwrap();
                    for k in hand {
                        if k == &'J' {
                            joker_hand.push(*highest.clone());
                        }
                        else {joker_hand.push(*k)}
                    }
                }
            }


        }
        joker_hand
    }

    fn card_value_to_type (card: &char) -> Cards {
        match card {
            'A' => Cards::A,
            'K' => Cards::K,
            'Q' => Cards::Q,
            'J' => Cards::J,
            'T' => Cards::T,
            '9' => Cards::Nine,
            '8' => Cards::Eight,
            '7' => Cards::Seven,
            '6' => Cards::Six,
            '5' => Cards::Five,
            '4' => Cards::Four,
            '3' => Cards::Three,
            _   => Cards::Two,
        }
    }


    // process data
    let mut full_set : Vec<Hand> = Vec::new();
    for row in &data {
        let hand: Hand = Hand {
            original: row[0].clone(),
            cards: row[0].chars().collect(),
            joker_cards: add_jokers(&row[0].chars().collect()),
            bid:string_to_u32(&row[1])};
        full_set.push(hand);
    }

    let mut total_score : u32 = 0;
    let mut id : u32 = 1;
    for hand in full_set.iter().sorted() {
        total_score += id * hand.bid;
        id += 1;

    }
    println!("Part 2: {total_score}");
    // 250845562 too low
    // 251037509 is correct - it would have been a lot quicker had I read the instructions!




    Ok(())
}