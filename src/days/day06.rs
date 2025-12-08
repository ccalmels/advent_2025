use std::io::{BufRead, Lines};

const WIDTH: usize = if cfg!(test) { 15 } else { 3745 };
const LINES: usize = if cfg!(test) { 4 } else { 5 };

fn resolve<T>(lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    let mut worksheet: [Vec<u8>; LINES] = Default::default();
    let (mut p1, mut p2) = (0, 0);

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for &c in line.as_bytes().iter() {
            worksheet[y].push(c);
        }
    }

    let operations = &worksheet[LINES - 1];

    let mut end = WIDTH;

    while let Some(index) = operations[..end].iter().rposition(|&c| c != b' ') {
        let p1_numbers = worksheet[0..LINES - 1].iter().map(|line| {
            line[index..end]
                .iter()
                .filter(|&&c| c != b' ')
                .fold(0, |acc, &c| acc * 10 + (c - b'0') as u64)
        });

        let p2_numbers = (index..end).filter_map(|x| {
            let number = worksheet[0..LINES - 1]
                .iter()
                .filter(|line| line[x] != b' ')
                .fold(0, |acc, line| acc * 10 + (line[x] - b'0') as u64);

            if number == 0 {
                None
            } else {
                Some(number)
            }
        });

        match operations[index] {
            b'+' => {
                p1 += p1_numbers.sum::<u64>();
                p2 += p2_numbers.sum::<u64>();
            }
            b'*' => {
                p1 += p1_numbers.product::<u64>();
                p2 += p2_numbers.product::<u64>();
            }
            _ => panic!("unknown operations {}", operations[index] as char),
        }

        end = index;
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (4277556, 3263827));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
