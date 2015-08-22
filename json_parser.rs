use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::collections::HashMap;
use std::fmt::Debug;

struct JSON {
    json: HashMap<String, JSONTypes>,
    index: usize,
    include_nan: bool
}
// this will help disseminate the various types expected in JSON

#[derive(Debug)]
enum JSONTypes {
    Obj { container: HashMap<String, JSONTypes> },
    Arr { container: Vec<JSONTypes> },
    Str(String),
    Int(i32),
    Bool(bool),
    Null,
    NaN
}

impl JSON {
    fn new(json: HashMap<String, JSONTypes>, index: usize, nan: bool) -> JSON {
        JSON { json: json, index: index, include_nan: nan }
    }

    fn load(&mut self, data: &mut String) {
        let buffer: Vec<char> = data.chars().collect();
        let buffer_length = buffer.len();
        while (self.index + 1) < buffer_length {
            self.index += 1;
            let key = self.parse_key(&buffer);
            let value = self.parse_value(&buffer);
            self.json.insert(key, value);
        }
    }

    fn parse_object(&mut self, buffer: &Vec<char>) -> JSONTypes {
        let mut json_obj: HashMap<String, JSONTypes> = HashMap::new();
        loop {
            let c = buffer.get(self.index).unwrap();
            if *c == '}' {
                self.index += 1;
                break;
            } else if *c == ',' {
                self.index += 1;
            } else {
                self.index += 1;
                let key: String = self.parse_key(buffer);
                let value: JSONTypes = self.parse_value(buffer);
                json_obj.insert(key, value);
            }
        }

        JSONTypes::Obj { container: json_obj }
    }

    fn parse_key(&mut self, buffer: &Vec<char>) -> String {
        self.index += 1;
        let mut c = buffer.get(self.index).unwrap();
        let mut key = String::new();
        while *c != '\"' {
            key.push(*c);
            self.index += 1;
            c = buffer.get(self.index).unwrap();
        }

        self.index += 1;

        if *buffer.get(self.index).unwrap() == ':' {
            self.index += 1;
        } else {
            self.index += 2;
        }

        key
    }

    fn parse_value(&mut self, buffer: &Vec<char>) -> JSONTypes {
        let c = buffer.get(self.index).unwrap();
        match *c {
            '{'  => self.parse_object(buffer),
            '\"' => self.parse_string(buffer),
            '['  => self.parse_array(buffer),
            't'  => self.parse_boolean(true),
            'f'  => self.parse_boolean(false),
            'n'  => self.parse_null(),
            'N'  => self.parse_nan(),
             _   => {
                 if c.is_numeric() {
                     self.parse_number(buffer)
                 } else {
                     panic!("something went wrong with {}", c)
                 }
            }
        }
    }

    fn parse_string(&mut self, buffer: &Vec<char>) -> JSONTypes {
        self.index += 1;
        let mut c = buffer.get(self.index).unwrap();
        let mut string = String::new();
        loop {
            if *c == '\"' {
                self.index += 1;
                break;
            }

            if *c == '\\' {
                self.index += 1;
                let escape = buffer.get(self.index).unwrap();
                match *escape {
                    'n'  => string.push('\n'),
                    'r'  => string.push('\r'),
                    '"'  => string.push('\"'),
                     _   => string.push(*escape)
                }
                self.index += 1;
            } else {
                string.push(*c);
                self.index += 1;
            }

            c = buffer.get(self.index).unwrap();
        }
        JSONTypes::Str(string)
    }

    fn parse_array(&mut self, buffer: &Vec<char>) -> JSONTypes {
        self.index += 1;
        let mut array: Vec<JSONTypes> = vec![];

        loop {
            let c = buffer.get(self.index).unwrap();
            if *c == ']' {
                self.index += 1;
                break;
            } else if *c == ',' {
                self.index += 1;
            } else {
                let value: JSONTypes = self.parse_value(buffer);
                array.push(value);
            }
        }

        JSONTypes::Arr { container: array }
    }

    fn parse_boolean(&mut self, value: bool) -> JSONTypes {
        if value {
            self.index += 4;
        } else {
            self.index += 5;
        }

        JSONTypes::Bool(value)
    }

    fn parse_null(&mut self) -> JSONTypes {
        self.index += 4;
        JSONTypes::Null
    }

    fn parse_nan(&mut self) -> JSONTypes {
        if self.include_nan {
            self.index += 3;
            JSONTypes::NaN
        } else {
            JSONTypes::Null
        }
    }

    fn parse_number(&mut self, buffer: &Vec<char>) -> JSONTypes {
        let mut container = String::new();
        let mut n = buffer.get(self.index).unwrap();

        while n.is_numeric() {
            container.push(*n);
            self.index += 1;
            n = buffer.get(self.index).unwrap();
        }

        let number = container.parse::<i32>();
        if number.is_ok() {
            JSONTypes::Int(number.unwrap())
        } else {
            panic!("Number parser failed on {}", container)
        }
    }

    // ACCESSOR METHODS AFTER LOAD

    fn keys(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = vec![];
        for key in self.json.keys() {
            keys.push(key);
        }

        keys
    }
}

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut data = String::new();
    // load the json data into `data`
    file.read_to_string(&mut data).unwrap();
    // instantiate the parser
    let mut json: JSON = JSON::new(HashMap::new(), 0, false);
    json.load(&mut data);

    for key in json.keys() {
        println!("key: {}", key);
    }

    // load buffer into parser
}
