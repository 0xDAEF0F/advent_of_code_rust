use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Down(String),
}

#[derive(Debug)]
enum Asset {
    #[allow(dead_code)]
    File { name: String, size: u64 },
    #[allow(dead_code)]
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

    pub fn root_size(&self) -> u64 {
        match &self.root {
            Asset::Directory { children, .. } => Self::go(&children),
            Asset::File { .. } => panic!("Root must be a directory."),
        }
    }

    fn go(asset: &HashMap<String, Asset>) -> u64 {
        let mut total_size: u64 = 0;
        for (_key, value) in asset {
            match value {
                Asset::File { size, .. } => total_size += size,
                Asset::Directory { children, .. } => {
                    let size = Self::go(children);
                    total_size += size;
                }
            }
        }

        total_size
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
                if child_dir == "/" {
                    self.current_path.clear()
                } else {
                    // verify it exists first
                    self.current_path.push(child_dir);
                }
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
                panic!("Path does not exist");
            }
        } else {
            panic!("Not a directory");
        }
    }
}

pub fn day_7_a(s: String) {
    let mut groups: Vec<Vec<String>> = vec![];

    for line in s.lines() {
        if line.starts_with("$") {
            groups.push(vec![line.to_string()]);
        } else {
            groups.last_mut().unwrap().push(line.to_string());
        }
    }

    let mut file_system = MyFileSystem::new();

    for group in groups {
        let mut group_iter = group.iter();
        let instruction = &group_iter.next().unwrap();

        if instruction.starts_with("$ ls") {
            for element in group_iter {
                if element.starts_with("dir") {
                    let name = element.split_whitespace().nth(1).unwrap();
                    file_system.add_directory(name.to_owned());
                } else {
                    let mut el_iter = element.split(' ');
                    let size = el_iter.next().unwrap().parse::<u64>().unwrap();
                    let name = el_iter.next().unwrap();
                    file_system.add_file(name.to_owned(), size);
                }
            }
        } else if instruction.starts_with("$ cd") {
            let direction = match instruction.split(' ').nth(2).unwrap() {
                ".." => Direction::Up,
                wh => Direction::Down(wh.to_owned()),
            };

            file_system.change_directory(direction);
        }
    }

    println!("{:?}", file_system.root_size());
}
