use std::io;

#[derive(Debug)]
struct Record<'a> {
    name: Option<&'a str>,
    length: usize,
    left: usize,
    right: usize,
    up: usize,
    down: usize,
}

impl<'a> Record<'a> {
    fn new(name: &'a str, i: usize) -> Self {
        if i == 0 {
            panic!("Record cannot be initialized if index is 0.");
        }

        Record {
            name: Some(name),
            length: 0,
            left: i - 1,
            right: i + 1,
            up: i,
            down: i,
        }
    }

    fn new_unnamed(left: usize, right: usize) -> Self {
        Record {
            name: None,
            length: 0,
            left,
            right,
            up: 0,
            down: 0,
        }
    }

    fn set_left(&mut self, left: usize) {
        self.left = left;
    }

    fn set_right(&mut self, right: usize) {
        self.right = right;
    }
}

fn main() {
    let mut buffer = String::new();

    let (primary_items, secondary_items) = loop {
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");

        if buffer.trim().is_empty() {
            continue;
        }

        if !buffer.is_ascii() {
            panic!("Item names should belong to ASCII range.");
        }

        if buffer.matches('|').count() > 1 {
            panic!("Item type separator \'|\' can only appear once.");
        }

        let mut items = buffer.split('|');

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

    // TODO: check that item names do not repeat

    let n1 = primary_items.len();
    let n2 = secondary_items.len();
    let n = n1 + n2;

    let mut items: Vec<Record> = Vec::with_capacity(n + 2);

    items.push(Record::new_unnamed(n1, 1));

    let mut i = 1;

    for item in primary_items {
        items.push(Record::new(item, i));
        i += 1;
    }

    items[n1].set_right(0);

    for item in secondary_items {
        items.push(Record::new(item, i));
        i += 1;
    }

    items.push(Record::new_unnamed(n, n1 + 1));

    items[n1 + 1].set_left(n + 1);

    for item in items {
        println!("{:?}", item);
    }
}
