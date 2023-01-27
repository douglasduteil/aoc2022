//

mod crt;
mod instruction;

//

use crt::Crt;
use instruction::Instruction;

//

pub fn part_one(input: &str) -> Option<isize> {
    const SIGNAL_STRENGTH: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let x_cycles: Vec<isize> = input
        .lines()
        .map(Instruction::from)
        .flat_map(|instr| instr.to_x_cycles())
        .collect();

    SIGNAL_STRENGTH
        .iter()
        .map(|&cycle| cycle as isize * (x_cycles.iter().take(cycle - 1).sum::<isize>() + 1))
        .map(Some)
        .sum()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut crt = Crt::default();

    for instruction in input.lines().map(Instruction::from) {
        crt.render(instruction.to_x_cycles());
    }
    Some(format!("{}", crt))
}
