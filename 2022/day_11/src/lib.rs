//

mod monkey;
mod note;

//

use crate::note::Note;

//

pub fn part_one(input: &str) -> Option<usize> {
    const ROUNDS: usize = 20;

    let mut note: Note = input.parse().expect("Note parsing error");

    let Note(ref monkeys) = note;
    let mut monkey_inspection_count = vec![0; monkeys.len()];

    for _ in 0..ROUNDS {
        let Note(ref mut monkeys) = note;
        for index in 0..monkeys.len() {
            let monkey = monkeys[index].clone();
            let throws_items = monkey.inspect_and_throw_items();
            monkey_inspection_count[monkey.id] += throws_items.len();
            monkeys[monkey.id].items.clear();
            for (throw_index, item) in throws_items {
                monkeys[throw_index].items.push(item);
            }
        }
    }

    monkey_inspection_count.sort();
    monkey_inspection_count.reverse();

    Some(monkey_inspection_count[0..2].iter().product())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}
