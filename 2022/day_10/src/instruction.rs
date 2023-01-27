//

use std::{convert::Infallible, str::FromStr};

//

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Addx(isize),
    Noop,
}

impl Instruction {
    pub fn to_x_cycles(&self) -> Vec<isize> {
        match self {
            // `addx V` takes *two cycles* to complete.
            // *After* two cycles, the `X` register is increased by the value `V`.
            Instruction::Addx(number) => vec![0, *number],
            // `noop` takes *one cycle* to complete. It has no other effect.
            Instruction::Noop => vec![0],
        }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}
impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once(" ") {
            Some((_, number)) => Ok(Instruction::Addx(number.parse().unwrap())),
            None => Ok(Instruction::Noop),
        }
    }
}

#[cfg(test)]
mod test_instruction_from_str {
    use super::*;

    #[test]
    fn test_noop() {
        assert_eq!("noop".parse(), Ok(Instruction::Noop))
    }

    #[test]
    fn test_addx_3() {
        assert_eq!("addx 3".parse(), Ok(Instruction::Addx(3)))
    }

    #[test]
    fn test_addx_sub5() {
        assert_eq!("addx -5".parse(), Ok(Instruction::Addx(-5)))
    }
}

#[cfg(test)]
mod test_instruction_to_x_cycles {
    use super::*;

    #[test]
    fn test_noop() {
        assert_eq!(Instruction::Noop.to_x_cycles(), [0]);
    }

    #[test]
    fn test_addx_3() {
        assert_eq!(Instruction::Addx(3).to_x_cycles(), [0, 3]);
    }
}

#[cfg(test)]
mod test_instruction_20th_example {
    use super::*;

    const CYCLE: usize = 20;

    #[test]
    fn test_20th_example() {
        let instructions: Vec<Instruction> = advent_of_code::read_example()
            .lines()
            .take(CYCLE)
            .flat_map(Instruction::from_str)
            .collect();

        let first_cycles = instructions
            .iter()
            .flat_map(|instr| instr.to_x_cycles())
            .take(CYCLE)
            .collect::<Vec<_>>();

        assert_eq!(1isize + first_cycles.iter().sum::<isize>(), 21);
    }
}

#[cfg(test)]
mod test_instruction_60th_example {
    use super::*;

    const CYCLE: usize = 60;

    #[test]
    fn test_noop() {
        let instructions: Vec<Instruction> = advent_of_code::read_example()
            .lines()
            .take(CYCLE)
            .flat_map(Instruction::from_str)
            .collect();

        let first_cycles = instructions
            .iter()
            .flat_map(|instr| instr.to_x_cycles())
            .take(CYCLE)
            .collect::<Vec<_>>();

        assert_eq!(1isize + first_cycles.iter().sum::<isize>(), 19);
    }
}
