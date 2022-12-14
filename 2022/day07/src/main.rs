use std::collections::HashMap;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct File {
    size: u64,
}

#[derive(Debug)]
struct Directory {
    path: PathBuf,
    contents: Vec<String>,
}

#[derive(Debug)]
enum DirEntry {
    File(File),
    Dir(Directory),
}

impl DirEntry {
    fn is_directory(&self) -> bool {
        matches!(self, DirEntry::Dir(_))
    }

    fn size(&self, fs: &FileSystem) -> u64 {
        match self {
            DirEntry::File(f) => f.size,
            DirEntry::Dir(d) => d
                .contents
                .iter()
                .filter_map(|child_name| fs.get(&d.path.join(child_name)))
                .map(|child| child.size(fs))
                .sum(),
        }
    }
}

#[derive(Debug)]
enum CommandLine {
    Cd(String),
    Ls,
    Dir(String),
    File(String, u64),
}

impl FromStr for CommandLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_ascii_whitespace().collect();
        match words[..] {
            ["$", "cd", dirname] => Ok(CommandLine::Cd(dirname.to_string())),
            ["$", "ls"] => Ok(CommandLine::Ls),
            ["dir", name] => Ok(CommandLine::Dir(name.to_string())),
            [size, name] => size
                .parse::<u64>()
                .map(|size| CommandLine::File(name.to_string(), size))
                .map_err(|_| format!("Invalid commandline: {}", s)),
            _ => Err(format!("Invalid commandline: {}", s)),
        }
    }
}

type FileSystem = HashMap<PathBuf, DirEntry>;

fn insert_direntry<F: FnOnce() -> DirEntry>(fs: &mut FileSystem, path: PathBuf, direntry: F) {
    if !fs.contains_key(&path) {
        fs.insert(path.clone(), direntry());

        match fs.get_mut(&path.parent().unwrap().to_path_buf()).unwrap() {
            DirEntry::Dir(d) => d
                .contents
                .push(path.file_name().unwrap().to_string_lossy().to_string()),
            _ => panic!("Parent of {} is not a directory", path.display()),
        }
    }
}

fn main() {
    let root_path: PathBuf = PathBuf::from("/");
    let mut fs = FileSystem::new();
    fs.insert(
        root_path.clone(),
        DirEntry::Dir(Directory {
            path: root_path.clone(),
            contents: vec![],
        }),
    );

    let _ = io::stdin()
        .lines()
        .map(|result| result.expect("I/O error"))
        .map(|line| line.parse::<CommandLine>().expect("Invalid commandline!"))
        .fold(PathBuf::from("/"), |mut pwd, command| match command {
            CommandLine::Cd(name) => match &name[..] {
                ".." => {
                    pwd.pop();
                    pwd
                }
                "/" => root_path.clone(),
                _ => {
                    let new_pwd = pwd.join(name);
                    insert_direntry(&mut fs, new_pwd.clone(), || {
                        DirEntry::Dir(Directory {
                            path: new_pwd.clone(),
                            contents: vec![],
                        })
                    });

                    new_pwd
                }
            },
            CommandLine::Dir(name) => {
                let dir_path = pwd.join(&name);
                insert_direntry(&mut fs, dir_path.clone(), || {
                    DirEntry::Dir(Directory {
                        path: dir_path.clone(),
                        contents: vec![],
                    })
                });

                pwd
            }
            CommandLine::File(name, size) => {
                let file_path = pwd.join(&name);
                insert_direntry(&mut fs, file_path, || DirEntry::File(File { size }));

                pwd
            }
            CommandLine::Ls => pwd,
        });

    let part_1: u64 = fs
        .values()
        .filter(|direntry| direntry.is_directory())
        .map(|dir| dir.size(&fs))
        .filter(|size| size < &100_000u64)
        .sum();

    println!("Part 1: {}", part_1);

    let total_used = fs.get(&root_path).unwrap().size(&fs);
    let unused = 70_000_000 - total_used;
    let minimum_needed = 30_000_000 - unused;

    let part_2: u64 = fs
        .values()
        .filter(|direntry| direntry.is_directory())
        .map(|dir| dir.size(&fs))
        .filter(|size| size >= &minimum_needed)
        .min()
        .expect("No solution to part 2 found?");

    println!("Part 2: {}", part_2);
}
