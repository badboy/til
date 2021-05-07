use std::process;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};
use std::fs::File;

use walkdir::WalkDir;
use xshell::{cmd, read_file};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

#[derive(Debug, Clone)]
struct Entry {
    path: PathBuf,
    title: String,
    mtime: String,
}

#[derive(Debug, Clone)]
struct Category {
    path: PathBuf,
    title: String,
    entries: Vec<Entry>
}

fn try_main() -> Result<()> {
    let walk = WalkDir::new("src")
        .min_depth(1) // skip `src/` itself
        .contents_first(false) // ensure we get the directory first
        .sort_by_file_name();

    let mut categories: BTreeMap<String, Category> = BTreeMap::new();
    let mut total_entries = 0;

    let ignore = [Path::new("src/README.md"), Path::new("src/SUMMARY.md")];

    for entry in walk {
        let entry = entry.unwrap();

        if ignore.iter().any(|path| path == &entry.path()) {
            continue;
        }

        let path = entry.path();

        if entry.file_type().is_dir() {
            let file = path.file_name().and_then(|s| s.to_str()).unwrap();
            let category = Category {
                path: path.strip_prefix("src/")?.to_path_buf(),
                title: camelize(file),
                entries: vec![],
            };
            categories.insert(file.to_string(), category);
            continue;
        }

        if path.ends_with("index.md") {
            continue;
        }

        let parent = entry.path().parent().ok_or("can't find parent directory")?;
        let category = parent.file_name().and_then(|s| s.to_str()).unwrap();

        let mtime = get_last_modification(entry.path())?;
        let title = get_title(entry.path())?;
        let trimmed_path = entry.path().strip_prefix("src/")?;
        let entry = Entry {
            path: trimmed_path.to_path_buf(),
            title: title,
            mtime: mtime,
        };

        let c = categories.get_mut(category).unwrap();
        c.entries.push(entry);
        total_entries += 1;
    }

    let header = read_file("templates/summary_head.md")?;
    let footer = read_file("templates/summary_footer.md")?;

    let mut file = BufWriter::new(File::create("./src/SUMMARY.md")?);
    write!(file, "{}", header)?;
    for (_, category) in categories {
        let path = Path::new("src").join(&category.path).join("index.md");
        let mut index = BufWriter::new(File::create(path)?);
        writeln!(index, "# {}\n", category.title)?;

        writeln!(file, "- [{}]({}/index.md)", category.title, category.path.display())?;
        for entry in category.entries {
            writeln!(file, "  - [{}]({})", entry.title, entry.path.display())?;
            writeln!(index, "- [{}]({}) - {}", entry.title, entry.path.display(), entry.mtime)?;
        }
    }
    write!(file, "{}", footer)?;

    let summary_start = "<!-- summary start -->\n";
    let summmary_end = "\n<!-- summary end -->";
    let content = read_file("./src/README.md")?;
    let (head, rest) = content.split_once(summary_start).ok_or("missing summary line start")?;
    let (_, rest) = rest.split_once(summmary_end).ok_or("missing summary line end")?;

    let mut file = BufWriter::new(File::create("./src/README.md")?);
    write!(file, "{}", head)?;
    write!(file, "{}", summary_start)?;
    write!(file, "So far there are {} TILs.", total_entries)?;
    write!(file, "{}", summmary_end)?;
    write!(file, "{}", rest)?;

    cmd!("mdbook build").run()?;

    Ok(())
}

fn get_last_modification(path: &Path) -> Result<String> {
    let mtime = cmd!("git --no-pager log -1 --pretty='format:%ci' {path}").read()?;

    match mtime.split_once(" ") {
        Some((date, _)) => Ok(date.to_string()),
        None => Err("no date found")?,
    }
}

fn get_title(path: &Path) -> Result<String> {
    let content = read_file(path)?;

    match content.split_once("\n") {
        Some((title, _)) => Ok(title.trim_start_matches("# ").to_string()),
        None => Err("no title found")?,
    }
}

fn camelize(title: &str) -> String {
    let mut c = title.chars();
    let first = c.next().unwrap().to_uppercase();
    let rest: String = c.collect();

    format!("{}{}", first, rest)
}
