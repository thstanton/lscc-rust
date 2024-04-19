use std::io::Write;
use std::{
    env,
    fs::{self, File},
};

// struct Element {
//     name: String,
//     attr: String,
//     children: Vec<Element>,
// }

fn main() {
    // // Read input from stdin
    // let mut input = String::new();
    // std::io::stdin().read_to_string(&mut input).unwrap();

    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let input = fs::read_to_string(file).unwrap();

    // let mut reader = Reader::from_str(&input);
    // reader.trim_text(true);
    // let mut count = 0;
    // let mut txt = Vec::new();
    // loop {
    //     match reader.read_event().unwrap() {
    //         Event::Start(e) => count += 1,
    //         Event::Text(e) => txt.push(e.unescape().unwrap().into_owned()),
    //         Event::Eof => break,
    //         _ => (),
    //     }
    // }

    // Deserialise XML to a struct
    let value: serde_json::Value = quick_xml::de::from_str(&input).unwrap();

    // Serialise the struct to JSON
    let output = serde_json::to_string_pretty(&value).unwrap();

    // Print to stdout
    println!("{output}");
    let mut destination_file = File::create("output.json").unwrap();
    destination_file.write_all(output);
}
