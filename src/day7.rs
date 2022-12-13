use std::collections::HashMap;
use std::collections::VecDeque;

type Arena = Vec<Entry>;
type Inode = usize;

pub fn solve(mut input: VecDeque<&str>) -> usize {
    let fs = FileSystem::new();
    let mut interpreter = Interpreter { fs };
    interpreter.traverse(&mut input);
    interpreter.estimate_cleanup_space()
}

pub fn part2(mut input: VecDeque<&str>) -> usize {
    let fs = FileSystem::new();
    let mut interpreter = Interpreter { fs };
    interpreter.traverse(&mut input);
    interpreter.part2()
}

#[derive(PartialEq, Debug)]
enum Type {
    File,
    Dir,
}

#[derive(Debug)]
struct Entry {
    name: String,
    size: usize,
    address: Inode,
    entry_type: Type,
}

impl Entry {
    fn new_dir(name: &str, address: Inode) -> Self {
        Entry {
            name: name.to_owned(),
            size: 0,
            address,
            entry_type: Type::Dir,
        }
    }

    fn new_file(name: &str, address: Inode, size: usize) -> Self {
        Entry {
            name: name.to_owned(),
            size,
            address,
            entry_type: Type::File,
        }
    }
}

struct FileSystem {
    arena: Arena,
    cwd: Inode,
    child_to_parent: HashMap<Inode, Inode>,
}

impl FileSystem {
    fn new() -> Self {
        let mut arena: Arena = Vec::new();
        let root = Entry::new_dir("/", 0);
        arena.push(root);

        FileSystem {
            cwd: 0,
            arena,
            child_to_parent: HashMap::new(),
        }
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            self.cwd = *self.child_to_parent.get(&self.cwd).unwrap();
            return;
        }

        let children = self.ls_cwd();
        self.cwd = if let Some(entry) = children.get(dir) {
            entry.address
        } else {
            self.mkdir(dir)
        }
    }

    fn ls(&self, dir: Inode) -> HashMap<String, &Entry> {
        self.arena
            .iter()
            .filter(|entry| self.child_to_parent.get(&entry.address) == Some(&dir))
            .map(|entry| (entry.name.to_owned(), entry))
            .collect()
    }

    fn ls_cwd(&self) -> HashMap<String, &Entry> {
        self.ls(self.cwd)
    }

    fn touch(&mut self, name: &str, size: usize) -> Inode {
        let result = self.arena.len();
        self.arena.push(Entry::new_file(name, result, size));
        self.child_to_parent.insert(result, self.cwd);
        result
    }

    fn mkdir(&mut self, name: &str) -> Inode {
        let result = self.arena.len();
        self.arena.push(Entry::new_dir(name, result));
        self.child_to_parent.insert(result, self.cwd);
        result
    }

    fn du(&self, inode: Inode) -> usize {
        let entry = &self.arena[inode];
        match entry.entry_type {
            Type::File => entry.size,
            Type::Dir => {
                let children = self.ls(inode);
                let mut result = 0;
                for child in children.values() {
                    let size = self.du(child.address);
                    result += size;
                }
                result
            }
        }
    }

    fn estimate_cleanup_space(&self) -> usize {
        let mut result = 0;
        for entry in self.arena.iter() {
            let du = self.du(entry.address);
            if du <= 100000 && entry.entry_type == Type::Dir {
                result += du;
            }
        }
        result
    }

    pub fn part2(&self) -> usize {
        let required = 30000000;

        let taken = self.du(0);
        let need_to_free = taken - required;

        self.arena
            .iter()
            .filter(|item| item.entry_type == Type::Dir)
            .map(|item| self.du(item.address))
            .filter(|size| size >= &need_to_free)
            .min()
            .unwrap()
    }
}

struct Interpreter {
    fs: FileSystem,
}

impl Interpreter {
    fn traverse(&mut self, input: &mut VecDeque<&str>) {
        while !input.is_empty() {
            let line = input.pop_front().unwrap();
            if line.starts_with('$') {
                let mut tokens = line.split(' ');
                let cmd = tokens.nth(1).unwrap();
                if cmd == "cd" {
                    let target_dir = tokens.next().unwrap();
                    self.fs.cd(target_dir);
                } else if cmd == "ls" {
                    self.tranverse_ls(input);
                }
            }
        }
    }

    fn tranverse_ls(&mut self, input: &mut VecDeque<&str>) {
        while !input.is_empty() {
            let head = input.pop_front();
            if let Some(fr) = head {
                if !fr.starts_with('$') {
                    self.track_child(fr);
                } else {
                    input.push_front(fr);
                    break;
                }
            }
        }
    }

    fn track_child(&mut self, line: &str) {
        let mut tokens = line.split(' ');
        let size = tokens.next().unwrap();
        let name = tokens.next().unwrap();
        self.create_child(name, size);
    }

    fn create_child(&mut self, name: &str, size_or_dir_flag: &str) -> Inode {
        let children = self.fs.ls_cwd();
        if size_or_dir_flag == "dir" {
            if children.contains_key(name) {
                return children.get(name).unwrap().address;
            } else {
                return self.fs.mkdir(name);
            }
        }
        if children.contains_key(name) {
            children.get(name).unwrap().address
        } else {
            let size = size_or_dir_flag
                .parse()
                .unwrap_or_else(|_| panic!("create_child: failed to parse {}", size_or_dir_flag));
            self.fs.touch(name, size)
        }
    }

    fn estimate_cleanup_space(&self) -> usize {
        self.fs.estimate_cleanup_space()
    }

    fn part2(&self) -> usize {
        self.fs.part2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_solve() {
        let input = vec![
            "cd /",
            "$ ls",
            "dir a",
            "1484854 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        assert_eq!(solve(input.into()), 95437);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "cd /",
            "$ ls",
            "dir a",
            "1484854 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];
        assert_eq!(part2(input.into()), 24933642);
    }

    #[test]
    fn test_part2_with_real_data() {
        let data = util::read_real_data("day7");
        let data = data.iter().map(|line| line.as_str()).collect();

        assert_eq!(part2(data), 14381780);
    }

    #[test]
    fn test_with_real_data() {
        let data = util::read_real_data("day7");
        let data = data.iter().map(|line| line.as_str()).collect();

        assert_eq!(solve(data), 1427048);
    }
}
