use std::io::{BufRead, Lines};

// This function tries to merge range with the last element of the
// vector. The idea here is that the vector is sorted, and range will
// be added at the end.
fn push_merge(v: &mut Vec<(u64, u64)>, range: (u64, u64)) {
    if let Some(last) = v.last_mut() {
        if last.1 + 1 >= range.0 {
            last.1 = last.1.max(range.1);
            return;
        }
    }
    v.push(range);
}

#[test]
fn check_push_merge() {
    let mut v = vec![(0, 2)];

    push_merge(&mut v, (4, 5));

    assert_eq!(v, [(0, 2), (4, 5)]);

    push_merge(&mut v, (5, 6));

    assert_eq!(v, [(0, 2), (4, 6)]);

    push_merge(&mut v, (7, 8));

    assert_eq!(v, [(0, 2), (4, 8)]);

    push_merge(&mut v, (5, 6));

    assert_eq!(v, [(0, 2), (4, 8)]);
}

// This function generates a vector of range by adding and merging a
// new range. The vector is sorted so we first try to find where the
// range must be inserted. When found, let's try to merge with the
// previous element and then try to merge the remaining ranges.
fn add_and_merge(v: Vec<(u64, u64)>, range: (u64, u64)) -> Vec<(u64, u64)> {
    let mut ret = vec![];
    let index = v.iter().position(|(first, _)| first > &range.0);

    if let Some(i) = index {
        ret.extend(&v[..i]);
        push_merge(&mut ret, range);

        for j in i..v.len() {
            push_merge(&mut ret, v[j]);
        }
    } else {
        ret.extend(v);
        push_merge(&mut ret, range);
    }

    ret
}

#[test]
fn check_add_and_merge() {
    let mut v = vec![(0, 2)];

    v = add_and_merge(v, (3, 4));

    assert_eq!(v, vec![(0, 4)]);

    v = add_and_merge(v, (10, 12));

    assert_eq!(v, vec![(0, 4), (10, 12)]);

    v = add_and_merge(v, (10, 12));

    assert_eq!(v, vec![(0, 4), (10, 12)]);

    v = add_and_merge(v, (6, 8));

    assert_eq!(v, vec![(0, 4), (6, 8), (10, 12)]);

    v = add_and_merge(v, (5, 7));

    assert_eq!(v, vec![(0, 8), (10, 12)]);

    v = add_and_merge(v, (2, 9));

    assert_eq!(v, vec![(0, 12)]);

    v = add_and_merge(v, (15, 20));
    v = add_and_merge(v, (10, 14));

    assert_eq!(v, vec![(0, 20)]);
}

fn resolve<T>(lines: Lines<T>) -> (i32, u64)
where
    T: BufRead,
{
    let mut is_range: bool = true;
    let mut ranges = vec![];
    let mut p1 = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            is_range = false;
            continue;
        }

        if is_range {
            let rs: Vec<u64> = line.split('-').map(|x| x.parse::<u64>().unwrap()).collect();
            let (first, last) = (rs[0], rs[1]);

            ranges = add_and_merge(ranges, (first, last));
        } else {
            let ingredient = line.parse::<u64>().unwrap();

            if !ranges
                .iter()
                .all(|&(first, last)| ingredient < first || ingredient > last)
            {
                p1 += 1;
            }
        }
    }

    let p2 = ranges
        .into_iter()
        .fold(0, |total, (first, last)| total + last - first + 1);

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (3, 14));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
