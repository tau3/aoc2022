use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;

type Arena = Rc<RefCell<Vec<Box<dyn Entry>>>>;

pub fn solve(mut input: VecDeque<&str>) -> usize {
    let mut fs = FileSystem::new();
    fs.traverse(&mut input);
    fs.delete_candidates()
}

struct Dir {
    name: String,
    parent: usize,
    contents: HashMap<String, usize>,
    arena: Arena,
}

impl Dir {
    fn new(name: String, parent: usize, arena: Arena) -> Self {
        Dir {
            name,
            parent,
            contents: HashMap::new(),
            arena,
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

    fn name(&self) -> &str {
        &self.name
    }

    fn add_child(&mut self, _: String, _: usize) {
        // do nothing
    }

    fn size(&self) -> usize {
        self.size
    }
}

impl Entry for Dir {
    fn parent(&self) -> usize {
        self.parent
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn add_child(&mut self, name: String, size: usize) {
        self.contents.insert(name, size);
    }

    fn size(&self) -> usize {
        let mut result = 0;
        for index in self.contents.values() {
            let item = &self.arena.borrow()[*index];
            let size = item.size();
            result += size;
        }
        println!("size of {} is {}", self.name, result);
        result
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dir name={}, parent={}, contents={:?}",
            self.name, self.parent, self.contents
        )
    }
}

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "dir name={}, parent={}, size={}",
            self.name, self.parent, self.size
        )
    }
}

trait Entry: std::fmt::Debug {
    fn parent(&self) -> usize;

    fn name(&self) -> &str;

    fn add_child(&mut self, name: String, index: usize);

    fn size(&self) -> usize;
}

struct FileSystem {
    arena: Arena,
    cwd_index: usize,
    by_name: HashMap<String, usize>,
}

impl FileSystem {
    fn new() -> Self {
        let arena = Rc::new(RefCell::new(Vec::new()));
        let root = Box::new(Dir::new("/".to_owned(), 0, Rc::clone(&arena)));
        arena.borrow_mut().push(root);
        let mut by_name = HashMap::new();
        by_name.insert("/".to_owned(), 0);

        FileSystem {
            cwd_index: 0,
            arena,
            by_name,
        }
    }

    fn pwd(&self, dir: &str) -> String {
        let cwd = &self.arena.borrow()[self.cwd_index];
        let mut result = cwd.name().to_owned();
	if result != "/"{
            result.push('/');
	}
        result.push_str(dir);
        result
    }

    fn cd(&mut self, dir: &str) {
        if dir == ".." {
            let cwd = &self.arena.borrow()[self.cwd_index];
            self.cwd_index = cwd.parent();
            return;
        }
        let pwd = self.pwd(dir);
        if let Some(asd) = self.by_name.get(&pwd) {
            self.cwd_index = *asd;
        } else {
            let dir = Dir::new(pwd, self.cwd_index, Rc::clone(&self.arena));
            self.arena.borrow_mut().push(Box::new(dir));
            self.cwd_index = self.arena.borrow().len() - 1;
        }
	println!("cd {}, new cwd: {}", dir, self.arena.borrow()[self.cwd_index].name());
    }

    fn track_child(&mut self, line: &str) {
        let mut tokens = line.split(' ');
        let size = tokens.next().unwrap();
        let name = tokens.next().unwrap();
        let pwd = self.pwd(name);
        let index = self.create_child(name, size);
        let mut binding = self.arena.borrow_mut();
        let cwd = binding.get_mut(self.cwd_index).unwrap();
        println!("add {} as child to {}", pwd, cwd.name());
        (*cwd).add_child(pwd, index);
    }

    fn create_child(&mut self, name: &str, size: &str) -> usize {
        println!("create child {} {}", name, size);
        let pwd = self.pwd(name);
        if size == "dir" {
            if self.by_name.contains_key(&pwd) {
                *self.by_name.get(&pwd).unwrap()
            } else {
                self.mkdir(&pwd)
            }
        } else {
            self.touch(
                &pwd,
                self.cwd_index,
                size.parse()
                    .unwrap_or_else(|_| panic!("create_child: failed to parse {}", size)),
            )
        }
    }

    fn touch(&mut self, pwd: &str, cwd: usize, size: usize) -> usize {
	println!("touch {} pwd", pwd);
        self.arena
            .borrow_mut()
            .push(Box::new(File::new(pwd.to_owned(), cwd, size)));
        self.arena.borrow().len() - 1
    }

    fn mkdir(&mut self, pwd: &str) -> usize {
	println!("mkdir {}", pwd);
        let item = Box::new(Dir::new(
            pwd.to_owned(),
            self.cwd_index,
            Rc::clone(&self.arena),
        ));
        self.arena.borrow_mut().push(item);
        self.arena.borrow().len() - 1
    }

    // TODO is directory
    fn delete_candidates(&self) -> usize {
        println!("arena: {:?}", self.arena);
        let mut result = 0;
        for x in self.arena.borrow().iter() {
            if x.size() <= 100000 {
                result += x.size();
            }
        }
        result
    }

    fn traverse(&mut self, input: &mut VecDeque<&str>) {
        while !input.is_empty() {
            let line = input.pop_front().unwrap();
            println!("process {} ", line);
            if line.starts_with('$') {
                let mut tokens = line.split(' ');
                let cmd = tokens.nth(1).unwrap();
                if cmd == "cd" {
                    let target_dir = tokens.nth(0).unwrap();
                    self.cd(target_dir);
                } else if cmd == "ls" {
                    while !input.is_empty() {
                        let head = input.pop_front();
                        if let Some(fr) = head {
			    println!("ls entry: {}", fr);
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
