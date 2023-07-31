use std::{collections::HashMap, error::Error, fmt::Display, io::Write};

#[derive(Debug)]
pub enum MinixError {
    NotAFile(Path),
    NotAFolder(Path),
    DoesntExist(Path),
    InvalidAssign(String),
}

impl Display for MinixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotAFile(p) => write!(f, "the path {p} is not a valid file"),
            Self::NotAFolder(p) => write!(f, "the path {p} is not a valid folder"),
            Self::DoesntExist(p) => write!(f, "the path {p} does not exist"),
            Self::InvalidAssign(s) => write!(f, "the assignment {s} is invalid"),
        }
    }
}

impl Error for MinixError {}

pub type MinixResult<T> = Result<T, MinixError>;

#[derive(Debug)]
pub enum Element {
    File(File),
    Folder(Folder),
}

#[derive(Debug)]
pub struct File {
    content: String,
}

#[derive(Debug)]
pub struct Folder {
    elements: HashMap<String, Element>,
}

#[derive(Debug, Clone)]
pub struct Path {
    elements: Vec<String>,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/")?;
        for elem in &self.elements {
            write!(f, "{}/", elem)?;
        }
        Ok(())
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self {
            elements: value.split('/').skip(1).map(|s| s.into()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct FileSystem {
    root: Folder,
}

impl FileSystem {
    fn get_file(&self, path: Path) -> MinixResult<&File> {
        let p2 = path.clone();
        let mut p_iter = p2.elements.iter();

        let mut loc = self
            .root
            .elements
            .get(p_iter.next().ok_or(MinixError::NotAFile(path.clone()))?)
            .ok_or(MinixError::DoesntExist(path.clone()))?;

        let mut final_file = None;
        for p in &mut p_iter {
            match loc {
                Element::Folder(f) => {
                    loc = f
                        .elements
                        .get(p)
                        .ok_or(MinixError::DoesntExist(path.clone()))?;
                }
                Element::File(f) => {
                    final_file = Some(f);
                    break;
                }
            }
        }

        match final_file {
            Some(f) => Ok(f),
            None => match loc {
                Element::File(f) => Ok(f),
                Element::Folder(_) => Err(MinixError::NotAFile(path.clone())),
            },
        }
    }

    fn get_file_mut(&mut self, path: Path) -> MinixResult<&mut File> {
        let p2 = path.clone();
        let mut p_iter = p2.elements.iter();

        let mut loc = self
            .root
            .elements
            .get_mut(p_iter.next().ok_or(MinixError::NotAFile(path.clone()))?)
            .ok_or(MinixError::DoesntExist(path.clone()))?;

        let mut final_file = None;
        for p in &mut p_iter {
            match loc {
                Element::Folder(f) => {
                    loc = f
                        .elements
                        .get_mut(p)
                        .ok_or(MinixError::DoesntExist(path.clone()))?;
                }
                Element::File(f) => {
                    final_file = Some(f);
                    break;
                }
            }
        }
        if p_iter.next().is_some() {
            return Err(MinixError::NotAFile(path));
        }

        match final_file {
            Some(f) => Ok(f),
            None => Err(MinixError::NotAFile(path)),
        }
    }
}

#[derive(Debug)]
pub struct State {
    environment_variables: HashMap<String, String>,
    fs: FileSystem,
}

impl Default for State {
    fn default() -> Self {
        let mut init = Self {
            environment_variables: HashMap::new(),
            fs: FileSystem {
                root: Folder {
                    elements: HashMap::new(),
                },
            },
        };

        init.fs.root.elements.insert(
            "env".into(),
            Element::Folder(Folder {
                elements: HashMap::new(),
            }),
        );

        if let Element::Folder(f) = init.fs.root.elements.get_mut("env").unwrap() {
            f.elements.insert(
                "startup_config".into(),
                Element::File(File {
                    content: "CWD=/\nUSER_DIR=/usr/home/".into(),
                }),
            );
        }

        init
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::default();

    state.environment_variables = load_startup_config(&state)?;
    println!("{:?}", state.environment_variables);

    prompt(&mut state)?;

    Ok(())
}

fn load_startup_config(state: &State) -> MinixResult<HashMap<String, String>> {
    let mut res = HashMap::new();
    let start_cfg = state.fs.get_file("/env/startup_config".into()).unwrap();
    for l in start_cfg.content.lines() {
        let (lhs, rhs) = l.split_at(l.find('=').ok_or(MinixError::InvalidAssign(l.into()))?);
        res.insert(lhs.into(), rhs[1..].into());
    }
    Ok(res)
}

fn prompt(state: &mut State) -> Result<(), Box<dyn Error>> {
    let current_dir = state.environment_variables.get("CWD").unwrap();
    println!("{current_dir}");
    std::io::stdout().flush()?;

    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    let input: Vec<&str> = input.trim().split_ascii_whitespace().collect();

    match input[0] {
        "mkdir" => {}

        unknown => {
            println!("unknown command: {unknown}")
        }
    }

    Ok(())
}
