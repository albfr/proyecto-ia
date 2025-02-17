pub struct Record {
    pub name: Option<String>,
    pub length: usize,
    pub left: usize,
    pub right: usize,
}

pub struct Node {
    pub top: isize,
    pub up: usize,
    pub down: usize,
}

impl Record {
    pub fn new(name: &str, i: usize) -> Self {
        if i == 0 {
            panic!("Record cannot be initialized if index is 0.");
        }

        Record {
            name: Some(String::from(name)),
            length: 0,
            left: i - 1,
            right: i + 1,
        }
    }

    pub fn new_unnamed(left: usize, right: usize) -> Self {
        Record {
            name: None,
            length: 0,
            left,
            right,
        }
    }

    pub fn add_node(&mut self) {
        self.length += 1;
    }

    pub fn remove_node(&mut self) {
        self.length -= 1;
    }
}

impl Node {
    pub fn new(top: isize, up: usize, down: usize) -> Self {
        Node { top, up, down }
    }

    pub fn new_spacer() -> Self {
        Node {
            top: 0,
            up: 0,
            down: 0,
        }
    }

    pub fn new_header(i: usize) -> Self {
        Node {
            top: 0,
            up: i,
            down: i,
        }
    }
}
