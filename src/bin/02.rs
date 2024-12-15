advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    let checked: Vec<(usize, usize)> = input.iter().map(|x| check_letters(x)).collect();

    let two = checked.iter().fold(0, |sum, x| sum + x.0);
    let three = checked.iter().fold(0, |sum, x| sum + x.1);

    Some(two * three)
}

fn check_letters(id: &str) -> (usize, usize) {
    let mut vec = vec![];

    let mut two = 0;
    let mut three = 0;

    let v: Vec<char> = id.chars().collect();

    for c in v.iter() {
        if !vec.contains(c) {
            let count = v.iter().filter(|&x| x == c).count();

            if count == 2 && two == 0 {
                two += 1;
            } else if count == 3 && three == 0 {
                three += 1;
            }

            vec.push(*c);
        }
    }

    (two, three)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = parse(input);
    let (i1, i2) = find_similar(&input);

    let mut vec = vec![];

    let id1 = input[i1].chars();
    let mut id2 = input[i2].chars();

    for c in id1 {
        if c == id2.next().unwrap() {
            vec.push(c);
        }
    }

    Some(vec.iter().collect())
}

fn find_similar(id: &[String]) -> (usize, usize) {
    for i in 0..id.len() {
        for j in 0..id.len() {
            let b1: Vec<char> = id[i].chars().collect();
            let b2: Vec<char> = id[j].chars().collect();

            let mut diff = 0;

            'inner: for k in 0..b1.len() {
                if b1[k] != b2[k] {
                    if diff == 0 {
                        diff = 1;
                    } else {
                        diff = 2;
                        break 'inner;
                    }
                }
            }

            if diff == 1 {
                return (i, j);
            }
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("fgij".to_string()));
    }
}
