extern crate walkdir;
extern crate pretty_bytes;

use walkdir::WalkDir;
use pretty_bytes::converter::convert;
use std::path::Path;
use std::collections::BTreeMap;
use std::fs::{remove_dir_all, remove_file};
use std::fmt;

// DefaultFiles pruned.
//
// Copied from yarn (mostly).
static DEFAULT_FILES: &'static [&str] = &[
    "Makefile",
    "Gulpfile.js",
    "Gruntfile.js",
    "gulpfile.js",
    ".DS_Store",
    ".tern-project",
    ".gitattributes",
    ".editorconfig",
    ".eslintrc",
    "eslint",
    ".eslintrc.js",
    ".eslintrc.json",
    ".eslintignore",
    ".stylelintrc",
    "stylelint.config.js",
    ".stylelintrc.json",
    ".stylelintrc.yaml",
    ".stylelintrc.yml",
    ".stylelintrc.js",
    ".htmllintrc",
    "htmllint.js",
    ".lint",
    ".npmignore",
    ".jshintrc",
    ".flowconfig",
    ".documentup.json",
    ".yarn-metadata.json",
    ".travis.yml",
    "appveyor.yml",
    ".gitlab-ci.yml",
    "circle.yml",
    ".coveralls.yml",
    "CHANGES",
    "LICENSE.txt",
    "LICENSE",
    "license",
    "AUTHORS",
    "CONTRIBUTORS",
    ".yarn-integrity",
    ".yarnclean",
    "_config.yml",
    ".babelrc",
    ".yo-rc.json",
    "jest.config.js",
    "karma.conf.js",
    ".appveyor.yml",
    "tsconfig.json"
];

// DefaultDirectories pruned.
//
// Copied from yarn (mostly).
static DEFAULT_DIRECTORIES: &'static [&str] = &[
    "__tests__",
    "test",
    "tests",
    "powered-test",
    "docs",
    "doc",
    ".idea",
    ".vscode",
    "website",
    "images",
    "assets",
    "example",
    "examples",
    "coverage",
    ".nyc_output",
    ".circleci",
    ".github",
];

static DEFAULT_EXTENSIONS: &'static [&str] = &[
    ".markdown",
    ".md",
    ".ts",
    ".jst",
    ".coffee",
    ".tgz",
    ".swp",
];

// TODO: Make concurrent iterating

type MapForRemoval = BTreeMap<String, bool>;

fn to_map(s: &'static [&str]) -> MapForRemoval {
    s.iter().fold(BTreeMap::new(), |mut tree: MapForRemoval, n| {
        tree.insert(n.to_string(), true);
        tree
    })
}

struct Stats {
    size_removed: u64,
    files_total: u64,
    files_removed: u64,
}

impl fmt::Debug for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stats {{ size_removed: {}, files_total: {}, files_removed: {} }}", convert(self.size_removed as f64), self.files_total, self.files_removed)
    }
}

impl Stats {
    fn new() -> Stats {
        Stats {
            size_removed: 0,
            files_total: 0,
            files_removed: 0,
        }
    }

    fn dir(&mut self, dir_path: &Path) {
        let dir_path = dir_path.to_str().unwrap_or_default();
        let walker = WalkDir::new(dir_path).into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            let path = entry.path();
            let metadata = path.metadata().unwrap();
            self.size_removed += metadata.len();
            self.files_removed += 1;
            self.files_total += 1;
        }
    }
}

struct Pruner<'a> {
    dir: &'a str,
    dirs: MapForRemoval,
    files: MapForRemoval,
    exts: MapForRemoval,
}

impl<'a> Pruner<'a> {
    fn new(dir: &'a str, dirs: MapForRemoval, files: MapForRemoval, exts: MapForRemoval) -> Pruner<'a> {
        Pruner {
            dir,
            dirs,
            files,
            exts,
        }
    }

    fn prune(&self) -> Stats {
        let mut stats = Stats::new();
        for entry in WalkDir::new(self.dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            let should_prune = self.should_prune(&path);
            if !should_prune {
                continue;
            }
            if path.is_dir() {
                stats.dir(path);
                if remove_dir_all(path).is_err() {
                    panic!("Don't have permissions on folder deleting")
                }

                continue;
            }
            let metadata = path.metadata().unwrap();
            stats.files_removed += 1;
            stats.size_removed += metadata.len();
            if remove_file(path).is_err() {
                panic!("Don't have permissions on file deleting")
            }
        }
        stats
    }

    fn should_prune(&self, path: &Path) -> bool {
        let file_name = path.file_name().unwrap().to_str().unwrap_or_default().to_string();
        if path.is_dir() {
            return self.dirs.contains_key(&file_name);
        }
        self.files.contains_key(&file_name) || self.exts.contains_key(&file_name)
    }
}


fn main() {
    let p = Pruner::new(
        &"test_data/node_modules",
        to_map(DEFAULT_DIRECTORIES),
        to_map(DEFAULT_FILES),
        to_map(DEFAULT_EXTENSIONS),
    );
    let stats = p.prune();
    println!("{:?}", stats);
}
