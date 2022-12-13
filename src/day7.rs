use std::collections::HashMap;
use std::collections::VecDeque;

pub fn solve(input: Vec<&str>) -> u32 {
    0
}

struct Dir {
    name: String,
    parent: usize,
    contents: HashMap<String, usize>,
}

impl Dir {
    fn new(name: String, parent: usize) -> Self {
        Dir {
            name: name,
            parent: parent,
            contents: HashMap::new(),
        }
    }
}

struct File {
    name: String,
    parent: usize,
    size: usize,
}

impl File {
    fn new(name: String, parent: usize, size: usize) -> Self {
        File { name, parent, size }
    }
}

impl Entry for File {
    fn parent(&self) -> usize {
        self.parent
    }

    fn name(&self) -> String {
        self.name
    }

    fn add_child(&mut self, _: String, _: usize) {

        // do nothing
    }
}

impl Entry for Dir {
    fn parent(&self) -> usize {
        self.parent
    }

    fn name(&self) -> String {
        self.name
    }

    fn add_child(&mut self, name: String, size: usize) {
        self.contents.insert(name, size);
    }
}

trait Entry {
    fn parent(&self) -> usize;

    fn name(&self) -> String;

    fn add_child(&mut self, name: String, index: usize);
}

struct FileSystem {
    items: Vec<Box<dyn Entry>>,
    cwd_index: usize,
    by_name: HashMap<String, usize>,
}

impl FileSystem {
    fn new() -> Self {
        let items = vec![Box::new(Dir::new("/".to_owned() , 0) ) as Box<dyn Entry>];
        let mut by_name = HashMap::new();
        by_name.insert("/".to_owned(), 0);

        FileSystem {
            cwd_index: 0,
            items: items,
            by_name: by_name,
        }
    }

    fn pwd(&self, dir: &str) -> String {
        let cwd = self.items[self.cwd_index];
        let mut result = cwd.name().clone();
        result.push('/');
        result.push_str(dir);
        result
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            let cwd = self.items[self.cwd_index];
            self.cwd_index = cwd.parent();
        }
        let cwd = self.items[self.cwd_index];
        let pwd = self.pwd(dir);
        if let Some(asd) = self.by_name.get(&pwd) {
            self.cwd_index = *asd;
        } else {
            self.items.push(Box::new(Dir::new(pwd, self.cwd_index)));
            self.cwd_index = self.items.len() - 1;
        }
    }

    fn track_child(&mut self, line: &str) {
        let tokens = line.split(" ");
        let size = tokens.nth(0).unwrap();
        let name = tokens.nth(0).unwrap();
        let pwd = self.pwd(name);
        if size == "dir" {
            let index = if self.by_name.contains_key(&pwd) {
                *self.by_name.get(&pwd).unwrap()
            } else {
                self.mkdir(pwd)
            };
            let cwd = self.items[self.cwd_index];
            cwd.add_child(pwd, index);
        } else {
            let index = self.touch(pwd, self.cwd_index, size.parse().unwrap());
            let cwd = self.items[self.cwd_index];
            cwd.add_child(pwd, index);
        }
    }

    fn touch(&self, pwd: String, cwd: usize, size: usize) -> usize {
        self.items.push(Box::new(File::new(pwd, cwd, size)));
        self.items.len() - 1
    }

    fn mkdir(&mut self, pwd: String) -> usize {
        self.items.push(Box::new(Dir::new(pwd, self.cwd_index)));
        self.items.len() - 1
    }

    fn traverse(&mut self, input: &mut VecDeque<&str>) {
        while !input.is_empty() {
            let line = input.pop_front().unwrap();
            if line.starts_with("$") {
                let tokens = line.split(" ");
                let cmd = tokens.nth(1).unwrap();
                if cmd == "cd" {
                    let target_dir = tokens.nth(1).unwrap();
                    self.cd(target_dir);
                } else if cmd == "ls" {
                    while (!input.is_empty()) {
                        let head = input.pop_front();
                        if let Some(fr) = head {
                            if (!fr.starts_with("S")) {
                                let x = input.pop_front().unwrap();
                                self.track_child(x);
                            }
                        }
                    }
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
        let input = [
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
        assert_eq!(solve(input), 95437);
    }
}
