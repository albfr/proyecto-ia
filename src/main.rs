use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
struct Record {
    name: Option<String>,
    length: usize,
    left: usize,
    right: usize,
}

impl Record {
    fn new(name: &str, i: usize) -> Self {
        if i == 0 {
            panic!("Record cannot be initialized if index is 0.");
        }

        Record {
            name: Some(name.to_string()),
            length: 0,
            left: i - 1,
            right: i + 1,
        }
    }

    fn new_unnamed(left: usize, right: usize) -> Self {
        Record {
            name: None,
            length: 0,
            left,
            right,
        }
    }

    fn set_left(&mut self, left: usize) {
        self.left = left;
    }

    fn set_right(&mut self, right: usize) {
        self.right = right;
    }

    fn add_node(&mut self) {
        self.length += 1;
    }
}

#[derive(Debug)]
struct Node {
    top: isize,
    up: usize,
    down: usize,
}

impl Node {
    fn new(top: isize, up: usize, down: usize) -> Self {
        Node {
            top,
            up,
            down,
        }
    }

    fn new_spacer() -> Self {
        Node {
            top: 0,
            up: 0,
            down: 0,
        }
    }

    fn new_header(i: usize) -> Self {
        Node {
            top: 0,
            up: i,
            down: i,
        }
    }

    fn set_up(&mut self, up: usize) {
        self.up= up;
    }

    fn set_down(&mut self, down: usize) {
        self.down= down;
    }
}

fn main() {
    let mut item_buffer = String::new();

    let (primary_items, secondary_items) = loop {
        io::stdin()
            .read_line(&mut item_buffer)
            .expect("Failed to read line.");

        if item_buffer.trim().is_empty() {
            continue;
        }

        if !item_buffer.is_ascii() {
            panic!("Item names should belong to ASCII range.");
        }

        if item_buffer.matches('|').count() > 1 {
            panic!("Item type separator \'|\' can only appear once.");
        }

        let mut items = item_buffer.split('|');

        let primary: Vec<&str> = items.next().unwrap().split_whitespace().collect();

        let secondary: Vec<&str> = match items.next() {
            Some(s) => s.split_whitespace().collect(),
            None => Vec::new(),
        };

        if primary.is_empty() {
            panic!("Primary items are required.");
        }

        break (primary, secondary);
    };

    let n1 = primary_items.len();
    let n2 = secondary_items.len();
    let n = n1 + n2;

    let mut item_header: Vec<Record> = Vec::with_capacity(n + 2);
    let mut node_list: Vec<Node> = Vec::with_capacity(n + 2);

    let mut item_index = HashMap::new();

    item_header.push(Record::new_unnamed(n1, 1));
    node_list.push(Node::new_spacer());

    let mut i = 1;

    for item in primary_items {
        item_header.push(Record::new(item, i));
        node_list.push(Node::new_header(i));

        if item_index.contains_key(item) {
            panic!("Item names must be unique.");
        }

        item_index.insert(item, i);

        i += 1;
    }

    item_header[n1].set_right(0);

    for item in secondary_items {
        item_header.push(Record::new(item, i));
        node_list.push(Node::new_header(i));

        if item_index.contains_key(item) {
            panic!("Item names must be unique.");
        }

        item_index.insert(item, i);

        i += 1;
    }

    item_header.push(Record::new_unnamed(n, n1 + 1));
    node_list.push(Node::new_spacer());

    item_header[n1 + 1].set_left(n + 1);

    let mut m = 0;
    let mut spacer = n + 1;

    let mut option_buffer = String::new();

    loop {
        let read_bytes = io::stdin()
            .read_line(&mut option_buffer)
            .expect("Failed to read line.");

        if read_bytes == 0 {
            break;
        }

        if option_buffer.trim().is_empty() {
            option_buffer.clear();
            continue;
        }

        let mut option_items = HashSet::new();

        for item_name in option_buffer.split_whitespace() {
            if let Some(&i) = item_index.get(item_name) {
                if option_items.contains(&i) {
                    panic!("Options must contain unique items.");
                }

                option_items.insert(i);

                let u = node_list[i].up;
                let j = node_list.len();

                item_header[i].add_node();
                node_list.push(Node::new(i as isize, u, i));
                node_list[u].set_down(j);
                node_list[i].set_up(j);
            } else {
                panic!("Options must contain known items.");
            }
        }

        m += 1;
        let s = node_list.len();
        node_list.push(Node::new(-m, spacer + 1, 0));
        node_list[spacer].set_down(s - 1);
        spacer = s;

        option_buffer.clear();
    };
}
