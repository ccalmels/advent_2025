use std::io::{BufRead, Lines};

fn find_invalid(min: u64, min_number_of_digits: usize, max: u64, split: usize) -> Vec<u64> {
    let split_size;
    let mut value;
    let mut mul;
    let mut ret = vec![];

    if min_number_of_digits.is_multiple_of(split) {
        split_size = min_number_of_digits / split;
        mul = (0..split_size).fold(1, |acc, _| acc * 10);
        value = (1..split).fold(min, |acc, _| acc / mul);
    } else {
        split_size = min_number_of_digits.div_ceil(split);
        value = (0..split_size - 1).fold(1, |acc, _| acc * 10);
        mul = 10 * value;
    }

    loop {
        let n = (1..split).fold(value, |acc, _| acc * mul + value);

        if n > max {
            break;
        }

        if n >= min {
            // Store Invalid ID.
            ret.push(n);
        }

        value += 1;

        if value == mul {
            mul *= 10;
        }
    }

    ret
}

#[cfg(test)]
fn find_invalid_test(min: &[u8], max: &[u8], split: usize) -> Vec<u64> {
    let min_number_of_digits = min.len();
    let min = min.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as u64);
    let max = max.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as u64);

    find_invalid(min, min_number_of_digits, max, split)
}

#[test]
fn check_find_invalid() {
    let min = "11".as_bytes();
    let max = "22".as_bytes();

    assert_eq!(find_invalid_test(min, max, 2), vec![11, 22]);

    let min = "110".as_bytes();
    let max = "115".as_bytes();

    assert_eq!(find_invalid_test(min, max, 3), vec![111]);

    let min = "95".as_bytes();
    let max = "115".as_bytes();

    assert_eq!(find_invalid_test(min, max, 2), vec![99]);
    assert_eq!(find_invalid_test(min, max, 3), vec![111]);

    let min = "998".as_bytes();
    let max = "1012".as_bytes();

    assert_eq!(find_invalid_test(min, max, 2), vec![1010]);
    assert_eq!(find_invalid_test(min, max, 3), vec![999]);

    let min = "2121212118".as_bytes();
    let max = "2121212124".as_bytes();

    assert_eq!(
        [2, 3, 5].into_iter().fold(vec![], |mut acc, split| {
            for f in find_invalid_test(min, max, split).into_iter() {
                if !acc.contains(&f) {
                    acc.push(f)
                }
            }
            acc
        }),
        vec![2121212121]
    );

    let min = "1111110".as_bytes();
    let max = "1111112".as_bytes();

    assert_eq!(find_invalid_test(min, max, 2), vec![]);
    assert_eq!(find_invalid_test(min, max, 3), vec![]);
    assert_eq!(find_invalid_test(min, max, 5), vec![]);
    assert_eq!(find_invalid_test(min, max, 7), vec![1111111]);
}

fn resolve<T>(mut lines: Lines<T>) -> (u64, u64)
where
    T: BufRead,
{
    let line = lines.next().unwrap().unwrap();
    let (mut p1, mut p2) = (0, 0);

    for range in line.split(',') {
        let rs = range.split('-').collect::<Vec<_>>();
        let (min, max) = (rs[0].as_bytes(), rs[1].as_bytes());

        // 7 is the max prime number that can split the range values.
        assert!(min.len() < 11);
        assert!(max.len() < 11);

        let min_number_of_digits = min.len();
        let min = min.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as u64);
        let max = max.iter().fold(0, |acc, c| acc * 10 + (c - b'0') as u64);

        let halves = find_invalid(min, min_number_of_digits, max, 2);

        p1 += halves.iter().sum::<u64>();

        let rest = [3, 5, 7].into_iter().fold(halves, |mut acc, split| {
            for f in find_invalid(min, min_number_of_digits, max, split).into_iter() {
                if !acc.contains(&f) {
                    acc.push(f)
                }
            }
            acc
        });

        p2 += rest.into_iter().sum::<u64>();
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (1227775554, 4174379265));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
