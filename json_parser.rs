use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    let result = file.read_to_string(&mut buffer);
    let mut z = buffer.chars();
    let mut n = 0;
    let mut c = z.next().unwrap();
    let mut hash: HashMap<String, String> = HashMap::new();
    parse_object(&mut z, &mut hash);
    for (key, value) in &hash {
        println!("{} key {} val", key, value);
    }
    // println!("{}", buffer);

    // loop {
    //     n += 1;
    //     let mut c = z.next();
    //     if c.is_none() {
    //         break;
    //     }
    //
    //     if c.unwrap() == '{' {
    //         parse_object(&mut z);
    //     }
    // }
}

fn parse_string(chars: &mut Iterator<Item=char>) -> String {
    chars.next().unwrap(); // skip initial quote
    chars.take_while(|&x| x != '"' ).collect()
}

fn parse_int(chars: &mut Iterator<Item=char>) -> usize {
    let n = chars.take_while(|&x| x.is_numeric() ).collect();
    n.parse::<usize>().unwrap()
}

// fn parse_key(chars: &mut Iterator<Item=char>) -> String {
//
// }

fn parse_object(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>) {

}

fn parse_string_value(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>, key: String) {
    let value = chars.take_while(|&x| x != '"' ).collect();
    hash.insert(key, value);
}

fn parse_array(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>) {
    let next_object = chars.next().unwrap();

    if next_object == '{' {
        parse_object(chars, hash);
        //
        // let key = parse_key(chars);
        // println!("{}", key);
        // let next_object_type = chars.next().unwrap();
        // println!("{}", next_object_type);
    }
}
