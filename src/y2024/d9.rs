use parser::Parser;

pub fn part1(input: String) -> String {
    let mut disk = parse_input(input);

    let mut i = 0;
    let mut j = disk.len() - 1;
    while i < j {
        while i < j && disk[i] != -1 {
            i += 1;
        }
        while i < j && disk[j] == -1 {
            j -= 1;
        }
        disk.swap(i, j);
    }

    let mut sum: i64 = 0;
    for (i, file) in disk.iter().enumerate() {
        if file != &-1 {
            sum += i as i64 * *file;
        }
    }

    sum.to_string()
}

pub fn part2(input: String) -> String {
    let mut disk = parse_input2(input);

    let mut j = disk.len();
    while j > 0 {
        j -= 1;
        for i in 0..=j {
            if let (DiskPart::FreeSpace(space), DiskPart::File(file)) =
                (disk[i].clone(), disk[j].clone())
            {
                if space.free >= file.size {
                    disk[i] = DiskPart::File(file.clone());
                    disk[j] = DiskPart::new_free_space(file.size);
                    if space.free > file.size {
                        disk.push(DiskPart::new_free_space(space.free - file.size));
                        disk[i + 1..].rotate_right(1);
                    }
                }
            }
        }
    }

    let mut sum: usize = 0;
    let mut i: usize = 0;
    for file in disk.iter() {
        match file {
            DiskPart::File(file) => {
                for _ in 0..file.size {
                    sum += i * file.id;
                    i += 1;
                }
            }
            DiskPart::FreeSpace(space) => i += space.free,
        }
    }

    sum.to_string()
}

fn parse_input(input: String) -> Vec<i64> {
    let mut parser = Parser::new(&input);
    let mut file = 0;
    let mut add_file = true;
    let mut result = vec![];
    while !parser.is_done() {
        let size: usize = parser.pop().unwrap().to_string().parse().unwrap();
        for _i in 0..size {
            if add_file {
                result.push(file);
            } else {
                result.push(-1);
            }
        }
        if add_file {
            file += 1;
        }
        add_file = !add_file;
    }
    result
}

#[derive(Debug, Clone)]
struct File {
    id: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct FreeSpace {
    free: usize,
}

#[derive(Debug, Clone)]
enum DiskPart {
    File(File),
    FreeSpace(FreeSpace),
}

impl DiskPart {
    fn new_file(id: usize, size: usize) -> Self {
        Self::File(File { id, size })
    }

    fn new_free_space(free: usize) -> Self {
        Self::FreeSpace(FreeSpace { free })
    }
}

fn parse_input2(input: String) -> Vec<DiskPart> {
    let mut parser = Parser::new(&input);
    let mut id = 0;
    let mut add_file = true;
    let mut result = vec![];
    while !parser.is_done() {
        let size: usize = parser.pop().unwrap().to_string().parse().unwrap();
        if add_file {
            result.push(DiskPart::new_file(id, size));
            id += 1;
        } else {
            result.push(DiskPart::new_free_space(size));
        }
        add_file = !add_file;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let result = part1(INPUT.to_string());
        assert_eq!(result, "1928");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT.to_string());
        assert_eq!(result, "2858");
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part1_input() {
        assert_eq!(
            part1(include_str!("../../.data/y2024/d9.txt").to_string()),
            "6386640365805"
        );
    }

    #[test]
    #[cfg(feature = "test_input")]
    fn test_part2_input() {
        assert_eq!(
            part2(include_str!("../../.data/y2024/d9.txt").to_string()),
            "6423258376982"
        );
    }
}
