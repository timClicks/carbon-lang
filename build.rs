// //! generates a description of valid unicode code points for given scripts
//
// use std::env;
// use std::fs;
// use std::path::Path;
// use std::str::FromStr;
//
// #[derive(PartialEq, Eq, Clone)]
// struct Script {
//     name: String,
//     code_points: Vec<CodePoint>,
// }
//
// #[derive(Eq, PartialEq, Copy, Clone)]
// enum CodePoint {
//     Scalar(u16),
//     Range(u16, u16),
// }
//
// impl FromStr for CodePoint {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let numbers: Vec<_> = s.split("..").collect();
//
//         if numbers.len() == 1 {
//             let scalar = u16::from_str_radix(numbers[0], 16).unwrap();
//             Ok(CodePoint::Scalar(scalar))
//         } else {
//             let lower = u16::from_str_radix(numbers[0], 16).unwrap();
//             let upper = u16::from_str_radix(numbers[1], 16).unwrap();
//             Ok(CodePoint::Range(lower, upper))
//         }
//     }
// }
//
// fn main() {
//     println!("cargo:rerun-if-changed=scripts.txt");
//     println!("cargo:rerun-if-changed=build.rs");
//
//     let mut f = fs::File::open("src/scripts.txt").unwrap();
//     let mut contents = String::with_capacity(1024*10);
//     f.read_to_string(&mut contents);
//
//     for line in contents.lines() {
//         if line.starts_with("#") || line.is_empty() {
//             continue;
//         }
//
//         let parts: Vec<_> = line.split_ascii_whitespace().collect();
//         if let Ok(code_point) = parts.parse::<CodePoint>() {
//             println!("{:?}", code_point)
//         }
//     }
//
//     let out_dir = env::var_os("OUT_DIR").unwrap();
//     let dest_path = Path::new(&out_dir).join("unicode_scripts.rs");
//     fs::write(
//         &dest_path,
//         "pub fn message() -> &'static str {
//             \"Hello, World!\"
//         }
//         "
//     ).unwrap();
// }

fn main() {

}