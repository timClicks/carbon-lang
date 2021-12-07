//! generates a description of valid unicode code points for given scripts

use unicode_general_category::{get_general_category, GeneralCategory};

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

/// In Unicode jargon, "script" is a writing system (more or less).
#[derive(PartialEq, Eq, Clone)]
struct Script {
    name: String,
    code_points: HashSet<CodePointRange>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct CodePointRange(char,char);

impl FromStr for CodePointRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s.split("..").collect();

        if numbers.len() == 1 {
            let scalar = u32::from_str_radix(numbers[0], 16).unwrap();
            let code_point = char::from_u32(scalar).unwrap();

            Ok(CodePointRange(code_point, code_point))
        } else {
            let lower = u32::from_str_radix(numbers[0], 16).unwrap();
            let upper = u32::from_str_radix(numbers[1], 16).unwrap();

            Ok(CodePointRange(char::from_u32(lower).unwrap(), char::from_u32(upper).unwrap()))
        }
    }
}

struct CodePoints {
    range: CodePointRange,
    position: char,
}

impl CodePoints {
    fn new(range: CodePointRange) -> Self {
        CodePoints {
            range,
            position: range.0,
        }
    }
}

impl IntoIterator for CodePointRange {
    type Item = char;
    type IntoIter = CodePoints;

    fn into_iter(self) -> Self::IntoIter {
        CodePoints::new(self)
    }
}


impl Iterator for CodePoints {
    type Item = char;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position > self.range.1 {
            return None;
        }
        let to_return = self.position;
        self.position = char::from_u32((self.position as u32) + 1).unwrap();

        Some(to_return)
    }
}

struct CodePoint;

fn main() {
    println!("cargo:rerun-if-changed=scripts.txt");
    println!("cargo:rerun-if-changed=build.rs");

    // let mut f = fs::File::open("scripts.txt").unwrap();
    // let mut contents = String::with_capacity(1024*10);
    // f.read_to_string(&mut contents);

    let contents = include_str!("scripts.txt");

    let mut script_names: HashSet<String> = HashSet::new();
    // let mut ranges: Vec<String> = HashSet::new();

    for line in contents.lines() {
        // lines within the scripts.txt file have the following structure:
        //
        // 2028          ; Common # Zl       LINE SEPARATOR

        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        let parts: Vec<_> = line.split_ascii_whitespace().collect();

        // no control (Cx) or separators (Zx)
        // the docs explicitly warn against parsing the comments, but hey
        if parts[4].starts_with("C") || parts[4].starts_with("Z") {
            continue;
        }

        // if let Ok(code_point) = parts[0].parse::<CodePointRange>() {
        //     for pt in code_point {
        //         print!("{} ", pt)
        //     }
        //     println!();
        // }

        script_names.insert(parts[2].to_owned().replace("_", ""));
    }

    let mut script_names = script_names
        .into_iter()
        .collect::<Vec<_>>();
    script_names.sort();

    println!("enum ScriptName {{");
    for s in script_names {
        println!("\t{},", s);
    }
    println!("}}");

    // let out_dir = env::var_os("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir).join("unicode_scripts.rs");
    // fs::write(
    //     &dest_path,
    //     "pub fn message() -> &'static str {
    //         \"Hello, World!\"
    //     }
    //     "
    // ).unwrap();
}