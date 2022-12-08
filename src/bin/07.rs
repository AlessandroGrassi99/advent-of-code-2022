use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

type RcDir = Rc<RefCell<Dir>>;
type WeakDir = Weak<RefCell<Dir>>;

pub struct Dir {
    pub parent: Option<WeakDir>,
    pub name: String,
    pub inside_dirs: HashMap<String, RcDir>,
    pub inside_files: HashMap<String, File>,
}

pub struct File {
    pub parent: WeakDir,
    pub size: usize,
    pub name: String,
}

impl Dir {
    pub(crate) fn size(&self) -> usize {
        let dirs_size = self
            .inside_dirs
            .iter()
            .map(|(_, dir)| dir.try_borrow().unwrap().size())
            .sum::<usize>();

        let files_size = self
            .inside_files
            .iter()
            .map(|(_, file)| file.size)
            .sum::<usize>();

        dirs_size + files_size
    }

    pub(crate) fn all_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::new();

        self.inside_dirs
            .iter()
            .for_each(|(_, dir)| sizes.append(&mut dir.try_borrow().unwrap().all_sizes()));

        let curr_dir_size = self.size();
        sizes.push(curr_dir_size);

        sizes
    }
}

fn build_file_system(input: &str) -> RcDir {
    let file_system = Rc::new(RefCell::new(Dir {
        parent: None,
        name: "/".to_string(),
        inside_dirs: HashMap::new(),
        inside_files: HashMap::new(),
    }));
    let mut current_dir: WeakDir = Rc::downgrade(&file_system);

    input.split("$ ").skip(1).for_each(|block| {
        let (input, output) = block.split_once('\n').unwrap();

        match input.split_once(' ') {
            Some(("cd", "/")) => current_dir = Rc::downgrade(&file_system),
            Some(("cd", "..")) => {
                current_dir = {
                    let tmp = current_dir
                        .upgrade()
                        .unwrap()
                        .try_borrow()
                        .unwrap()
                        .parent
                        .as_ref()
                        .unwrap()
                        .upgrade()
                        .unwrap();

                    Rc::downgrade(&tmp)
                }
            }
            Some(("cd", dir_name)) => {
                current_dir = Rc::downgrade(
                    current_dir
                        .upgrade()
                        .unwrap()
                        .try_borrow()
                        .unwrap()
                        .inside_dirs
                        .get(dir_name)
                        .unwrap(),
                );
            }
            None => {
                let current_dir = current_dir.upgrade().unwrap();
                output
                    .lines()
                    .for_each(|line| match line.split_once(' ').unwrap() {
                        ("dir", dir_name) => {
                            let dir = Dir {
                                parent: Some(Rc::downgrade(&current_dir)),
                                name: dir_name.to_string(),
                                inside_dirs: HashMap::new(),
                                inside_files: HashMap::new(),
                            };
                            let mut tmp = current_dir.try_borrow_mut().unwrap();
                            tmp.inside_dirs
                                .insert(dir_name.to_string(), Rc::new(RefCell::new(dir)));
                        }
                        (size, file_name) => {
                            let file = File {
                                parent: Rc::downgrade(&current_dir),
                                size: size.parse::<usize>().unwrap(),
                                name: file_name.to_string(),
                            };
                            current_dir
                                .try_borrow_mut()
                                .unwrap()
                                .inside_files
                                .insert(file_name.to_string(), file);
                        }
                    });
            }
            Some((a, b)) => {
                println!("{} {}", a, b);

                unreachable!()
            }
        }
    });

    file_system
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_system = build_file_system(input);

    let all_sizes = file_system
        .try_borrow()
        .unwrap()
        .all_sizes();

    let filtered_sizes = all_sizes
        .into_iter()
        .filter(|x| *x <= 100000)
        .sum::<usize>();

    Some(filtered_sizes as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_system = build_file_system(input);

    let all_sizes = file_system
        .try_borrow()
        .unwrap()
        .all_sizes();

    let total_size = file_system.try_borrow().unwrap().size();
    let available_memory = 70000000 - total_size;
    let target_space = 30000000;

    Some(
        all_sizes.into_iter().fold(total_size, |acc, x| {
            match x < acc && available_memory + x >= target_space {
                true => x,
                false => acc,
            }
        }) as u32
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
