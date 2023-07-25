//
//
// pub fn load_json<T>(path : &str) ->T{
//
//     let mut file = std::fs::File::open(path).unwrap();
//     let mut stdout = std::io::stdout();
//     let mut str = &std::io::copy(&mut file, &mut stdout).unwrap().to_string();
//     serde_json::from_str(str).expect("JSON was not well-formatted")
// }