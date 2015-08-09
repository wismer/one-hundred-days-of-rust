use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

struct Object {
    container: HashMap<String, JPrimitive>,
    start_point: char,
    end_point: char
}

struct JInt {
    n: usize
}

struct Array {
    container: Vec<JPrimitive>,
    start_point: char,
    end_point: char
}

enum JPrimitive {
    Object,
    JArray(Array),
    JString,
    JBool,
    JNone,
    JInt
}

impl JPrimitive {
    fn show(&self) {
        println!("asdasfsdgsdfds");
    }
}

// struct JPrimitive {
//     array: Array
// }

trait ObjectParser {
    fn insert(&mut self, key: &str, value: JPrimitive);
    fn create_key(&mut self, iter: &mut Iterator<Item=char>) -> String;
    fn build_value(&mut self, iter: &mut Iterator<Item=char>, end_point: &char);
}

impl Array {
    fn hi_there(&self) {
        println!("asdfjhsdflgkjhsdflkjsdhflksdjhfslkdjgh");
    }
}

impl ObjectParser for Object {
    fn insert(&mut self, key: &str, value: JPrimitive) {
        self.container.insert(key.to_string(), value);
    }

    fn create_key(&mut self, iter: &mut Iterator<Item=char>) -> String {
        iter.take_while(|&c| c != ':' ).filter(|&c| c != '"' ).collect()
    }

    fn build_value(&mut self, iter: &mut Iterator<Item=char>, end_point: &char) {

    }
}

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    let mut data = file.read_to_string(&mut buffer);
    let mut chars = buffer.chars();

}

fn create_object(iter: &mut Iterator<Item=char>) -> JPrimitive {
    let mut obj = Object { container: HashMap::new(), start_point: '{', end_point: '}' };
    let key = obj.create_key(iter);
    iter.next().unwrap(); // discard expected :

    // let mut next_character: Option<char> = iter.next();


}
