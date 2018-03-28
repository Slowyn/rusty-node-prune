extern crate walkdir;

use walkdir::WalkDir;
use std::path::Path;
use std::collections::BTreeMap;

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

fn to_map(s: &'static [&str]) -> BTreeMap<String, bool> {
    s.iter().fold(BTreeMap::new(), |mut tree, n| {
        tree.insert(n.to_string(), true);
        tree
    })
}

struct Stats {
    size_removed: i64,
    files_total: i64,
    files_removed: i64,
}

type MapForRemoval = BTreeMap<String, bool>;

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

    fn prune(&self) {
        for entry in WalkDir::new(self.dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            let should_prune = self.should_prune(&path);
            let metadata = path.metadata().unwrap();
            if should_prune {
                println!("File should be pruned. Size: {:?}", metadata.len());
            }
        }
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
    p.prune();
}
