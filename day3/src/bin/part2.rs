use std::collections::HashMap;

use day3::{Grid, Index};
use util::read_full_input;

fn main() {
    let grid: Grid = read_full_input().parse().unwrap();
    let mut sum = 0;

    let mut gear_adjecensy: HashMap<Index, Vec<u32>> = HashMap::new();
    for (idx, num) in grid.nums() {
        let width = num.to_string().len();

        grid.adjacent_symbols(idx, width)
            .filter(|idx| grid.symbol_at(idx).is_some_and(|s| s == '*'))
            .for_each(|idx| {
                gear_adjecensy.entry(idx).or_default().push(num);
            });
    }

    dbg!(&gear_adjecensy);

    for adjacent_numbers in gear_adjecensy.values() {
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }

    println!("{sum}");
}
