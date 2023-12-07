use day3::Grid;
use util::read_full_input;

fn main() {
    let grid: Grid = read_full_input().parse().unwrap();

    let mut sum = 0;
    for (idx, num) in grid.nums() {
        let width = num.to_string().len();

        if grid.adjacent_symbols(idx, width).next().is_some() {
            sum += num;
        }
    }
    println!("{sum}");
}
