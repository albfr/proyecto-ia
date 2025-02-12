use std::collections::{HashMap, HashSet};

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

pub struct DancingLinks {
    item_header: Vec<Record>,
    node_list: Vec<Node>,
    item_index: HashMap<String, usize>,
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

    fn add_node(&mut self) {
        self.length += 1;
    }

    fn remove_node(&mut self) {
        self.length -= 1;
    }

    fn set_left(&mut self, l: usize) {
        self.left = l;
    }

    fn set_right(&mut self, r: usize) {
        self.right = r;
    }
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

    fn set_up(&mut self, u: usize) {
        self.up= u;
    }

    fn set_down(&mut self, d: usize) {
        self.down= d;
    }
}

impl DancingLinks {
    pub fn new(primary_items: Vec<&str>, secondary_items: Vec<&str>) -> Self {
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

            item_index.insert(item.to_string(), i);

            i += 1;
        }

        item_header[n1].set_right(0);

        for item in secondary_items {
            item_header.push(Record::new(item, i));
            node_list.push(Node::new_header(i));

            if item_index.contains_key(item) {
                panic!("Item names must be unique.");
            }

            item_index.insert(item.to_string(), i);

            i += 1;
        }

        item_header.push(Record::new_unnamed(n, n1 + 1));
        node_list.push(Node::new_spacer());

        item_header[n1 + 1].set_left(n + 1);

        DancingLinks {
            item_header,
            node_list,
            item_index,
        }
    }

    pub fn get_item(&self, i: usize) -> &Record {
        &self.item_header[i]
    }

    pub fn get_root_item(&self) -> &Record {
        self.get_item(0)
    }

    pub fn get_node(&self, i: usize) -> &Node {
        &self.node_list[i]
    }

    fn get_list_len(&self) -> usize {
        self.node_list.len()
    }

    fn add_node(&mut self, i: usize) {
        self.item_header[i].add_node();
    }

    fn remove_node(&mut self, i: usize) {
        self.item_header[i].remove_node();
    }

    fn set_left(&mut self, i: usize, l: usize) {
        self.item_header[i].set_left(l);
    }

    fn set_right(&mut self, i: usize, r: usize) {
        self.item_header[i].set_right(r);
    }

    fn set_up(&mut self, i: usize, u: usize) {
        self.node_list[i].set_up(u);
    }

    fn set_down(&mut self, i: usize, d: usize) {
        self.node_list[i].set_down(d);
    }

    pub fn add_option(&mut self, option_str: &str) {
        let spacer = self.get_list_len() - 1;
        let mut option_items = HashSet::new();

        for item_name in option_str.split_whitespace() {
            if let Some(&i) = self.item_index.get(item_name) {
                if option_items.contains(&i) {
                    panic!("Options must contain unique items.");
                }

                option_items.insert(i);

                let u = self.get_node(i).up;
                let j = self.get_list_len();

                self.add_node(i);
                self.node_list.push(Node::new(i.try_into().unwrap(), u, i));
                self.set_down(u, j);
                self.set_up(i, j);
            } else {
                panic!("Options must contain known items.");
            }
        }

        self.node_list.push(Node::new(self.get_node(spacer).top - 1, spacer + 1, 0));
        self.set_down(spacer, self.get_list_len() - 2);
    }

    fn hide(&mut self, p: usize) {
        let mut q = p + 1;

        while q != p {
            let node = self.get_node(q);
            let t = node.top;
            let u = node.up;
            let d = node.down;

            if t <= 0 {
                q = u;
            } else {
                self.set_down(u, d);
                self.set_up(d, u);
                self.remove_node(t.try_into().unwrap());
                q += 1;
            }
        }
    }

    fn unhide(&mut self, p: usize) {
        let mut q = p - 1;

        while q != p {
            let node = self.get_node(q);
            let t = node.top;
            let u = node.up;
            let d = node.down;

            if t <= 0 {
                q = d;
            } else {
                self.set_down(u, q);
                self.set_up(d, q);
                self.add_node(t.try_into().unwrap());
                q -= 1;
            }
        }
    }

    pub fn cover(&mut self, i: usize) {
        let mut p = self.get_node(i).down;

        while p != i {
            self.hide(p);
            p = self.get_node(p).down;
        }

        let l = self.get_item(i).left;
        let r = self.get_item(i).right;

        self.set_right(l, r);
        self.set_left(r, l);
    }

    pub fn uncover(&mut self, i: usize) {
        let l = self.get_item(i).left;
        let r = self.get_item(i).right;

        self.set_right(l, i);
        self.set_left(r, i);

        let mut p = self.get_node(i).up;

        while p != i {
            self.unhide(p);
            p = self.get_node(p).up;
        }
    }
}
