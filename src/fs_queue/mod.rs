use std::{
    collections::VecDeque,
    fs,
    sync::{mpsc::channel, Arc, Mutex, OnceLock},
    thread,
};

use walkdir::WalkDir;

use crate::{
    parse_tree::{
        parse::{error::ParserError, parse_root},
        ParseTree,
    },
    tokenizer::Tokenizer,
};

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: Arc<str>,
    pub src: Arc<str>,
}

#[derive(Debug, Clone)]
pub struct SourceError {
    pub source: SourceFile,
    pub err: ParserError,
}

static SOURCE_FILES: OnceLock<Mutex<VecDeque<SourceFile>>> = OnceLock::new();

fn is_gek(entry: &str) -> bool {
    return entry.ends_with(".gek");
}

pub fn deque_source() -> Option<SourceFile> {
    SOURCE_FILES.get().unwrap().lock().unwrap().pop_back()
}

pub fn load_all_source() {
    let walker = WalkDir::new("test").into_iter();

    let mut files = VecDeque::new();

    for entry in walker.filter_entry(|it| {
        it.file_type().is_dir() || it.file_name().to_str().map(is_gek).unwrap_or(false)
    }) {
        let Ok(entry) = entry else {
            continue;
        };

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(path) = entry.path().to_str() else {
            continue;
        };

        let Ok(src) = fs::read_to_string(path) else {
            continue;
        };

        files.push_front(SourceFile {
            path: path.into(),
            src: src.into(),
        });
    }

    SOURCE_FILES.set(Mutex::new(files)).unwrap();
}

fn parse_source() -> Result<Vec<ParseTree>, SourceError> {
    let mut trees = vec![];
    loop {
        let source = deque_source();

        let Some(source) = source else {
            break;
        };

        let mut tokenizer = Tokenizer::new(source.src.clone());

        let root = parse_root(&mut tokenizer);

        match root {
            Ok(root) => trees.push(root),
            Err(err) => return Err(SourceError { source, err }),
        }
    }

    return Ok(trees);
}

pub fn parse_trees() -> Result<(), SourceError> {
    let mut threads = vec![];
    let (tx, rx) = channel();
    for _ in 0..num_cpus::get() {
        let tx = tx.clone();
        threads.push(thread::spawn(move || tx.send(parse_source())));
    }
    for thread in threads {
        thread.join().unwrap().unwrap();
    }

    let mut trees = VecDeque::new();

    for tree in rx.iter().take(num_cpus::get()) {
        let mut tree = tree?.into();
        trees.append(&mut tree);
    }

    for tree in trees {
        println!("{:#?}", tree);
    }

    Ok(())
}
