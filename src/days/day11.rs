use std::collections::HashMap;
use std::io::{BufRead, Lines};

fn letters_to_u32(letters: &[u8]) -> u32 {
    letters.iter().fold(0, |acc, &c| (acc * 256) + c as u32)
}

fn word_to_u32(word: &str) -> u32 {
    letters_to_u32(word.as_bytes())
}

#[cfg(test)]
fn u32_to_string(n: u32) -> String {
    std::str::from_utf8(&[16u32, 8, 0].map(|v| ((n >> v) & 0xff) as u8))
        .unwrap()
        .into()
}

#[test]
fn check_conversion() {
    let n = word_to_u32("you");

    assert_eq!(n, 0x796f75);
    assert_eq!("you", u32_to_string(n));
}

fn get_paths_nb_cached(
    cables: &HashMap<u32, Vec<u32>>,
    start: u32,
    end: u32,
    cache: &mut HashMap<u32, Option<usize>>,
) -> usize {
    if let Some(entry) = cache.get(&start) {
        // We're already know this path.
        //
        // If entry contains a Some(path), let's return path. If it's
        // None it means we're cycling, so let's just break the cycle
        // and return 0 (default).
        entry.unwrap_or_default()
    } else {
        // First time we're here.
        //
        // Mark the cache with None. It means we're currently trying
        // to resolve it. It is used as a hint to break cycle.
        cache.insert(start, None);

        let paths = if start == end {
            1
        } else if let Some(v) = cables.get(&start) {
            v.iter()
                .map(|&next| get_paths_nb_cached(cables, next, end, cache))
                .sum()
        } else {
            // out?
            0
        };

        // The path is now resolved, let's update the cache.
        cache.insert(start, Some(paths));

        paths
    }
}

fn get_paths_nb(cables: &HashMap<u32, Vec<u32>>, start: u32, end: u32) -> usize {
    get_paths_nb_cached(cables, start, end, &mut HashMap::new())
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut cables = HashMap::new();

    for line in lines {
        let line = line.unwrap();
        let bytes = line.as_bytes();

        let entry = letters_to_u32(&bytes[0..3]);
        let outputs: Vec<u32> = bytes[5..]
            .split(|&c| c == b' ')
            .map(letters_to_u32)
            .collect();

        cables.insert(entry, outputs);
    }

    let devices: Vec<u32> = ["svr", "fft", "dac", "out"]
        .into_iter()
        .map(word_to_u32)
        .collect();

    (
        get_paths_nb(&cables, word_to_u32("you"), word_to_u32("out")),
        devices
            .iter()
            .zip(devices.iter().skip(1))
            .map(|(&a, &b)| get_paths_nb(&cables, a, b))
            .product::<usize>(),
    )
}

#[test]
fn check() {
    const TEST: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (5, 0));
}

#[test]
fn check_part2() {
    const TEST2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST2).lines()), (0, 2));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
