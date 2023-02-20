//

mod input;
mod monkey;
mod round;

//

use monkey::Item;

use crate::{input::Reader, round::Round};

//

pub fn part_one(input: &str) -> Option<u32> {
    let note: Reader = input.parse().expect("Note parsing error");
    let Reader(monkeys) = note;

    fn human_worry(Item(worry): Item) -> Item {
        Item(worry / 3)
    }

    let mut round = Round::new(&monkeys, human_worry);

    for _ in 0..20 {
        round.next();
    }

    Some(round.monkey_business() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let note: Reader = input.parse().expect("Note parsing error");
    let Reader(monkeys) = note;

    let monkeys_divisor = monkeys
        .iter()
        .map(|monkey| match monkey.test {
            monkey::Test::DivisibleBy(divisor, _) => divisor,
        })
        .product::<u64>();

    let human_worry = move |Item(worry): Item| -> Item { Item(worry % monkeys_divisor) };

    let mut round = Round::new(&monkeys, human_worry);

    for _ in 0..10_000 {
        round.next();
    }

    Some(round.monkey_business() as u32)
}
