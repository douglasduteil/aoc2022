//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<Item>,
    pub operation: Operation,
    pub test: Test,
}

impl Monkey {
    pub fn inspect_and_throw_items(&self) -> Vec<(usize, Item)> {
        let Monkey {
            items,
            test,
            operation,
            ..
        } = self;

        items
            .iter()
            .map(|&Item(worry)| {
                let worry = operation.calc(worry);
                let worry = worry / 3;

                let index = match test {
                    Test::DivisibleBy(divisor, (left, right)) => {
                        if worry % divisor == 0 {
                            *left
                        } else {
                            *right
                        }
                    }
                };

                (index, Item(worry))
            })
            .collect()
    }
}

//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Worry {
    #[default]
    Old,
    Level(usize),
}

impl Worry {
    fn or(&self, old_value: usize) -> usize {
        match self {
            Worry::Old => old_value,
            Worry::Level(value) => *value,
        }
    }
}

//

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Add(Worry, Worry),
    Multiply(Worry, Worry),
}

impl Operation {
    pub fn calc(&self, old_value: usize) -> usize {
        match self {
            Operation::Add(left, right) => left.or(old_value) + right.or(old_value),
            Operation::Multiply(left, right) => left.or(old_value) * right.or(old_value),
        }
    }
}

//

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Test {
    DivisibleBy(usize, (usize, usize)),
}

//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Item(pub usize);

//

#[cfg(test)]
mod test_monkey {
    use std::vec;

    use super::*;

    #[test]
    fn test_nothing() {
        let monkey = Monkey {
            id: 0,
            items: vec![],
            operation: Operation::Add(Worry::Old, Worry::Old),
            test: Test::DivisibleBy(1, (0, 0)),
        };

        assert_eq!(monkey.inspect_and_throw_items(), vec![])
    }

    #[test]
    fn test_inspect_and_throw_items() {
        let monkey = Monkey {
            id: 42,
            items: vec![Item(3)],
            operation: Operation::Add(Worry::Old, Worry::Old),
            test: Test::DivisibleBy(2, (3, 4)),
        };

        assert_eq!(monkey.inspect_and_throw_items(), vec![(3, Item(2))])
    }
}
