use std::collections::HashMap;

enum Direction {
    Up,
    Down(String),
}

#[derive(Debug)]
enum Asset {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: HashMap<String, Asset>,
    },
}

#[derive(Debug)]
struct MyFileSystem {
    root: Asset,
    current_path: Vec<String>,
}

impl MyFileSystem {
    pub fn new() -> Self {
        Self {
            root: Asset::Directory {
                name: "root".to_string(),
                children: HashMap::new(),
            },
            current_path: vec![],
        }
    }

    pub fn add_file(&mut self, name: String, size: u64) {
        let file = Asset::File {
            name: name.to_owned(),
            size,
        };
        MyFileSystem::insert_at_path(&mut self.root, &self.current_path, name, file);
    }

    pub fn add_directory(&mut self, name: String) {
        let directory = Asset::Directory {
            name: name.to_owned(),
            children: HashMap::new(),
        };
        MyFileSystem::insert_at_path(&mut self.root, &self.current_path, name, directory);
    }

    pub fn change_directory(&mut self, r#where: Direction) {
        match r#where {
            Direction::Up => {
                assert!(!self.current_path.is_empty());

                self.current_path.pop();
            }
            Direction::Down(child_dir) => {
                // verify it exists first

                self.current_path.push(child_dir);
            }
        }
    }

    fn insert_at_path(node: &mut Asset, path: &[String], name: String, asset: Asset) {
        if path.is_empty() {
            if let Asset::Directory { children, .. } = node {
                children.insert(name, asset);
            } else {
                panic!("Not a directory");
            }
            return;
        }

        let next = path[0].clone();
        if let Asset::Directory { children, .. } = node {
            if let Some(next_node) = children.get_mut(&next) {
                MyFileSystem::insert_at_path(next_node, &path[1..], name, asset);
            } else {
                println!("{:?}", path);
                panic!("Path does not exist");
            }
        } else {
            panic!("Not a directory");
        }
    }
}

pub fn day_7_a(s: String) {
    let mut groups: Vec<Vec<String>> = vec![vec!["$ cd /".to_string()]];

    for line in s.lines() {
        let last_group = groups.last_mut().unwrap();

        if !line.starts_with("$") {
            last_group.push(line.to_string())
        } else {
            groups.push(vec![line.to_string()])
        }
    }

    let mut file_system = MyFileSystem::new();

    for group in groups {
        let mut group_iter = group.iter();
        let instr = &group_iter.next().unwrap()[2..];

        if instr.starts_with("ls") {
            for element in group_iter {
                if element.starts_with("dir") {
                    let name = element.split(' ').next().unwrap();
                    file_system.add_directory(name.clone().to_string());
                } else {
                    let mut el_iter = element.split(' ');
                    let size = el_iter.next().unwrap().parse::<u64>().unwrap();
                    let name = el_iter.next().unwrap();
                    file_system.add_file(name.to_owned(), size);
                }
            }
        } else if instr.starts_with("cd") {
            let direction = match instr {
                ".." => Direction::Up,
                wh => Direction::Down(wh.to_owned()),
            };

            file_system.change_directory(direction);
        }
    }

    println!("{:?}", file_system);
}
