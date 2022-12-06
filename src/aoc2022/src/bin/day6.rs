use aoc_attributes::aoc_main;
use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day6.txt");

#[aoc_main(year = 2022, day = 6, part1 = "part1", part2 = "part2")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn part1() -> usize {
    find_marker(INPUT, 4)
}

fn find_marker(input: &str, chunk_size: usize) -> usize {
    input
        .as_bytes()
        .windows(chunk_size)
        .enumerate()
        .find_map(|(idx, chunk)| {
            if chunk.iter().unique().count() == chunk_size {
                Some(idx + chunk_size)
            } else {
                None
            }
        })
        .unwrap_or_default()
}
pub fn part2() -> usize {
    find_marker(INPUT, 14)
}

#[cfg(test)]
mod day6 {
    use crate::find_marker;

    #[test]
    fn test_part1() {
        let chunk_size = 4;
        assert_eq!(7, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", chunk_size));
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", chunk_size));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", chunk_size));
        assert_eq!(
            10,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", chunk_size)
        );
        assert_eq!(
            11,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", chunk_size)
        );
    }

    #[test]
    fn test_part2() {
        let chunk_size = 14;
        assert_eq!(
            19,
            find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", chunk_size)
        );
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", chunk_size));
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", chunk_size));
        assert_eq!(
            29,
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", chunk_size)
        );
        assert_eq!(
            26,
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", chunk_size)
        );
    }
}
