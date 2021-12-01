use itermore::IterMore;
use std::iter::Iterator;

pub(crate) fn prepare() -> impl Iterator<Item = usize> + Clone + 'static {
    let file = include_str!("../data/day1-input-1.txt");
    let data = file.lines().filter_map(|k| k.parse().ok());

    return data;
}

pub(crate) fn part1<D: Iterator<Item = usize>>(data: D) -> usize {
    data.windows()
        .map(|[current, next]| next > current)
        .filter(|&s| s)
        .count()
}

pub(crate) fn part2<D: Iterator<Item = usize>>(data: D) -> usize {
    let rolling_sums = data
        .windows()
        .map(|[current, next, nextnext]| current + next + nextnext);
    return part1(rolling_sums);
}

#[cfg(test)]
mod test {
    use super::part1;
    use super::part2;

    #[test]
    fn test_part1() {
        assert_eq!(part1([1, 2, 3].into_iter()), 2);
        assert_eq!(
            part1([199, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter()),
            7
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2([1, 2, 3].into_iter()), 0);
        assert_eq!(
            part2([199, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter()),
            5
        );
    }
}
