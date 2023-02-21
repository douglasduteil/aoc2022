//

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<Item>,
    pub operation: Operation,
    pub test: Test,
}

//

type WorryType = u64;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Worry {
    #[default]
    Old,
    Level(WorryType),
}

impl Worry {
    fn or(&self, old_value: WorryType) -> WorryType {
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
    pub fn calc(&self, old_value: WorryType) -> WorryType {
        match self {
            Operation::Add(left, right) => left.or(old_value) + right.or(old_value),
            Operation::Multiply(left, right) => left.or(old_value) * right.or(old_value),
        }
    }
}

#[cfg(test)]
mod test_operation {
    use super::*;

    #[test]
    fn test_add_1_2() {
        let operation = Operation::Add(Worry::Level(1), Worry::Level(2));
        assert_eq!(operation.calc(42), 3)
    }

    #[test]
    fn test_add_old_1() {
        let operation = Operation::Add(Worry::Old, Worry::Level(1));
        assert_eq!(operation.calc(42), 43)
    }

    #[test]
    fn test_multiply_1_2() {
        let operation = Operation::Multiply(Worry::Level(1), Worry::Level(2));
        assert_eq!(operation.calc(42), 2)
    }

    #[test]
    fn test_multiply_old_2() {
        let operation = Operation::Multiply(Worry::Old, Worry::Level(2));
        assert_eq!(operation.calc(42), 84)
    }
}
//

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Test {
    DivisibleBy(WorryType, (usize, usize)),
}

impl Test {
    pub fn throw_index(&self, worry: u64) -> usize {
        match self {
            Test::DivisibleBy(divisor, (left, right)) => {
                if worry % *divisor == 0 {
                    *left
                } else {
                    *right
                }
            }
        }
    }
}

#[cfg(test)]
mod test_test {
    use super::*;

    #[test]
    fn test_8_throw_divisible_by_3_index_1() {
        let test = Test::DivisibleBy(3, (0, 1));
        assert_eq!(test.throw_index(8), 1)
    }

    #[test]
    fn test_9_throw_divisible_by_3_index_0() {
        let test = Test::DivisibleBy(3, (0, 1));
        assert_eq!(test.throw_index(9), 0)
    }
}

//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Item(pub WorryType);

//
