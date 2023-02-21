//

use std::collections::HashMap;

use crate::monkey::{Item, Monkey};

pub struct Round {
    human_worry: Box<dyn Fn(Item) -> Item>,
    monkey_by_id: HashMap<usize, Monkey>,
    monkey_inspection_count: HashMap<usize, u64>,
}

//

impl Round {
    pub fn new(monkeys: &[Monkey], human_worry: impl Fn(Item) -> Item + 'static) -> Self {
        let monkey_by_id = monkeys.iter().map(|m| (m.id, m.clone())).collect();
        Self {
            human_worry: Box::new(human_worry),
            monkey_by_id,
            monkey_inspection_count: monkeys.iter().map(|m| (m.id, 0)).collect(),
        }
    }

    pub fn monkey_business(&self) -> u64 {
        let mut monkey_inspection_count: Vec<u64> =
            self.monkey_inspection_count.values().copied().collect();

        monkey_inspection_count.sort_unstable();

        monkey_inspection_count.iter().rev().take(2).product()
    }
}

//

impl Iterator for Round {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let Round { human_worry, .. } = self;

        for id in 0..self.monkey_by_id.len() {
            let monkey = self
                .monkey_by_id
                .get_mut(&id)
                .expect(" Monkey {monkey_id} not found");

            let throws_items = monkey
                .items
                .drain(..)
                .map(|item| {
                    let item = Item(monkey.operation.calc(item.0));
                    let item = human_worry(item);
                    let throw_to_monkey_id = monkey.test.throw_index(item.0);
                    (throw_to_monkey_id, item)
                })
                .collect::<Vec<_>>();

            self.monkey_inspection_count
                .entry(monkey.id)
                .and_modify(|count| *count += throws_items.len() as u64);

            for (throw_index, item) in throws_items {
                self.monkey_by_id
                    .entry(throw_index)
                    .and_modify(|monkey| monkey.items.push(item));
            }
        }
        Some(())
    }
}

#[cfg(test)]
mod test_round {
    use crate::monkey::{Operation, Test, Worry};

    use super::*;

    #[test]
    fn test_first_round() {
        let monkeys = monkey_fixtures();
        let mut round = Round::new(&monkeys, |Item(worry)| Item(worry / 3));

        round.next();

        assert_eq!(round.monkey_inspection_count.get(&0), Some(&2));
        assert_eq!(round.monkey_inspection_count.get(&1), Some(&4));
        assert_eq!(round.monkey_inspection_count.get(&2), Some(&3));
        assert_eq!(round.monkey_inspection_count.get(&3), Some(&5));
    }

    //

    fn monkey_fixtures() -> Vec<Monkey> {
        [
            Monkey {
                id: 0,
                items: vec![Item(79), Item(98)],
                operation: Operation::Multiply(Worry::Old, Worry::Level(19)),
                test: Test::DivisibleBy(23, (2, 3)),
            },
            Monkey {
                id: 1,
                items: vec![Item(54), Item(65), Item(75), Item(74)],
                operation: Operation::Add(Worry::Old, Worry::Level(6)),
                test: Test::DivisibleBy(19, (2, 0)),
            },
            Monkey {
                id: 2,
                items: vec![Item(79), Item(60), Item(97)],
                operation: Operation::Multiply(Worry::Old, Worry::Old),
                test: Test::DivisibleBy(13, (1, 3)),
            },
            Monkey {
                id: 3,
                items: vec![Item(74)],
                operation: Operation::Add(Worry::Old, Worry::Level(3)),
                test: Test::DivisibleBy(17, (0, 1)),
            },
        ]
        .to_vec()
    }
}
