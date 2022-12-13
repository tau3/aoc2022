use std::collections::HashMap;
use std::collections::VecDeque;

type Arena = Vec<Entry>;
type Inode = usize;

pub fn solve(mut input: VecDeque<&str>) -> usize {
    let mut fs = FileSystem::new();
    fs.traverse(&mut input);
    fs.delete_candidates()
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
        let children = self.children(self.cwd);
        if children.contains_key(dir) {
            let inode = children.get(dir).unwrap().address;
            self.cwd = inode;
        } else {
            self.cwd = self.mkdir(dir);
        }
    }

    fn track_child(&mut self, line: &str) {
        let mut tokens = line.split(' ');
        let size = tokens.next().unwrap();
        let name = tokens.next().unwrap();
        self.create_child(name, size);
    }

    fn children(&self, dir: Inode) -> HashMap<String, &Entry> {
        self.arena
            .iter()
            .filter(|x| self.child_to_parent.get(&x.address) == Some(&dir))
            .map(|e| (e.name.to_owned(), e))
            .collect()
    }

    fn create_child(&mut self, name: &str, size_or_dir_flag: &str) -> Inode {
        let children = self.children(self.cwd);
        if size_or_dir_flag == "dir" {
            if children.contains_key(name) {
                return children.get(name).unwrap().address;
            } else {
                self.mkdir(name)
            }
        } else {
            if children.contains_key(name) {
                return children.get(name).unwrap().address;
            }
            self.touch(
                name,
                size_or_dir_flag.parse().unwrap_or_else(|_| {
                    panic!("create_child: failed to parse {}", size_or_dir_flag)
                }),
            )
        }
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
        let result = match entry.entry_type {
            Type::File => entry.size,
            Type::Dir => {
                let children = self.children(inode);
                let mut result = 0;
                for child in children.values() {
                    let size = self.du(child.address);
                    result += size;
                }
                result
            }
        };
        result
    }

    fn delete_candidates(&self) -> usize {
        let mut result = 0;
        for entry in self.arena.iter() {
            let du = self.du(entry.address);
            if du <= 100000 && entry.entry_type == Type::Dir {
                result += du;
            }
        }
        result
    }

    fn traverse(&mut self, input: &mut VecDeque<&str>) {
        while !input.is_empty() {
            let line = input.pop_front().unwrap();
            if line.starts_with('$') {
                let mut tokens = line.split(' ');
                let cmd = tokens.nth(1).unwrap();
                if cmd == "cd" {
                    let target_dir = tokens.next().unwrap();
                    self.cd(target_dir);
                } else if cmd == "ls" {
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
                } else {
                    panic!("unknown cmd {}", cmd);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
