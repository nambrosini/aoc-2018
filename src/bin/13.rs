advent_of_code::solution!(13);

fn parse(input: &str) -> Vec<Vec<u8>> {
    // Have to do this since the input is trimmed automatically by cargo-aoc!
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn part_one(input: &str) -> Option<String> {
    let tracks = parse(input);
    Some(solve(&tracks, false))
}

fn part_two(input: &str) -> Option<String> {
    let tracks = parse(input);
    Some(solve(&tracks, true))
}

fn solve(tracks: &[Vec<u8>], remove: bool) -> String {
    let mut carts = carts(tracks);
    loop {
        carts.sort_by_key(|&(x, y, _, _)| (y, x));
        for (i, (x, y, dir, turns)) in carts.clone().into_iter().enumerate() {
            if carts[i].0 == usize::MAX {
                continue;
            }
            let (mut nx, ny) = match dir {
                0 => (x, y - 1),
                1 => (x + 1, y),
                2 => (x, y + 1),
                _ => (x - 1, y),
            };
            let (ndir, nturns) = match tracks[ny][nx] {
                b'/' => ([1, 0, 3, 2][dir as usize], turns),
                b'\\' => ([3, 2, 1, 0][dir as usize], turns),
                b'+' => match turns {
                    0 => ((dir + 3) % 4, 1),
                    1 => (dir, 2),
                    _ => ((dir + 1) % 4, 0),
                },
                _ => (dir, turns),
            };
            for (j, (ref mut ox, oy, _, _)) in carts.iter_mut().enumerate() {
                if i != j && nx == *ox && ny == *oy {
                    if remove {
                        *ox = usize::MAX;
                        nx = usize::MAX;
                        break;
                    } else {
                        return format!("{},{}", nx, ny);
                    }
                }
            }
            carts[i] = (nx, ny, ndir, nturns);
        }
        carts.retain(|&(x, _, _, _)| x != usize::MAX);
        if remove && carts.len() == 1 {
            return format!("{},{}", carts[0].0, carts[0].1);
        }
    }
}

// Dir: ^ > v  <
fn carts(tracks: &[Vec<u8>]) -> Vec<(usize, usize, u8, u8)> {
    tracks
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().flat_map(move |(x, c)| match *c {
                b'^' => Some((x, y, 0, 0)),
                b'>' => Some((x, y, 1, 0)),
                b'v' => Some((x, y, 2, 0)),
                b'<' => Some((x, y, 3, 0)),
                _ => None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("7,3".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some("6,4".to_string()));
    }
}
