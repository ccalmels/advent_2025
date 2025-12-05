use std::io::{BufRead, Lines};

// This function returns the first max.
//
// The max functions in iterator are always returning the last one so it is not equivalent to:
//
// let index = &bank[start_index..bank.len() - k]
//     .iter()
//     .enumerate()
//     .max_by_key(|&(_, item)| item)
//     .unwrap();
fn find_max(slice: &[u8]) -> (usize, u8) {
    let mut max = 0;
    let mut index = 0;

    for (i, &v) in slice.iter().enumerate() {
        if v > max {
            index = i;
            max = v;
        }
    }

    (index, max)
}

fn find_joltage(bank: &[u8], batteries_number: usize) -> u64 {
    let mut number = 0;
    let mut start_index = 0;

    for k in (0..batteries_number).rev() {
        let value = find_max(&bank[start_index..bank.len() - k]);

        start_index += value.0 + 1;
        number = number * 10 + (value.1 - b'0') as u64;
    }

    number
}

fn resolve<T>(lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    lines.fold((0, 0), |(p1, p2), line| {
        let line = line.unwrap();
        let bank = line.as_bytes();

        (p1 + find_joltage(bank, 2), p2 + find_joltage(bank, 12))
    })
}

#[test]
fn check() {
    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (357, 3121910778619));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
