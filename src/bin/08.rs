advent_of_code::solution!(8);

fn parse(input: &str) -> Node {
    let mut input: Vec<usize> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let header = (input.remove(0), input.remove(0));
    let mut children = vec![];

    for _ in 0..header.0 {
        children.push(get_node(&mut input));
    }

    Node::new(header, children, input.clone())
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(count_metadata(&input))
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input);
    Some(get_value(&input))
}

fn count_metadata(node: &Node) -> usize {
    let mut metadata = node.metadata.iter().sum();

    for n in &node.children {
        metadata += count_metadata(n);
    }

    metadata
}

fn get_node(input: &mut Vec<usize>) -> Node {
    let header = (input.remove(0), input.remove(0));
    let mut children = vec![];

    for _ in 0..header.0 {
        children.push(get_node(input));
    }

    let mut metadata = vec![];

    for _ in 0..header.1 {
        metadata.push(input.remove(0));
    }

    Node::new(header, children, metadata)
}

fn get_value(node: &Node) -> usize {
    if node.header.0 == 0 {
        return node.metadata.iter().sum();
    }

    let mut sum = 0;

    for i in &node.metadata {
        if *i > node.children.len() {
            continue;
        }
        sum += get_value(&node.children[*i - 1]);
    }

    sum
}

pub struct Node {
    header: (usize, usize),
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new(header: (usize, usize), children: Vec<Node>, metadata: Vec<usize>) -> Node {
        Self {
            header,
            children,
            metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
