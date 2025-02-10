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

    fn remove_node(&mut self) {
        self.length -= 1;
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

fn hide(p: usize, item_header: &mut [Record], node_list: &mut [Node]) {
    let mut q = p + 1;

    while q != p {
        let node = &node_list[q];
        let x = node.top;
        let u = node.up;
        let d = node.down;

        if x <= 0 {
            q = u;
        } else {
            node_list[u].set_down(d);
            node_list[d].set_up(u);
            item_header[x as usize].remove_node();
            q += 1;
        }
    }
}

fn unhide(p: usize, item_header: &mut [Record], node_list: &mut [Node]) {
    let mut q = p - 1;

    while q != p {
        let node = &node_list[q];
        let x = node.top;
        let u = node.up;
        let d = node.down;

        if x <= 0 {
            q = d;
        } else {
            node_list[u].set_down(q);
            node_list[d].set_up(q);
            item_header[x as usize].add_node();
            q -= 1;
        }
    }
}

fn cover(i: usize, item_header: &mut [Record], node_list: &mut [Node]) {
    let mut p = node_list[i].down;

    while p != i {
        hide(p, item_header, node_list);
        p = node_list[p].down; 
    }

    let l = item_header[i].left;
    let r = item_header[i].right;

    item_header[l].set_right(r);
    item_header[r].set_left(l);
}

fn uncover(i: usize, item_header: &mut [Record], node_list: &mut [Node]) {
    let l = item_header[i].left;
    let r = item_header[i].right;

    item_header[l].set_right(i);
    item_header[r].set_left(i);

    let mut p = node_list[i].up;

    while p != i {
        unhide(p, item_header, node_list);
        p = node_list[p].up; 
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
                let j = node_list.len() as usize;

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

    let mut level = 0;
    let mut solution = Vec::new();
    let mut exit_level = false;

    loop {
        let check_exit = exit_level;
        exit_level = false;

        if item_header[0].right != 0 && !check_exit {
            let mut min_length = usize::MAX;
            let mut p = item_header[0].right;
            i = p;

            while p != 0 {
                let length = item_header[p].length;

                if length < min_length {
                    min_length = length;
                    i = p;

                    if min_length == 0 {
                        break;
                    }
                }

                p = item_header[p].right;
            }

            cover(i, &mut item_header, &mut node_list);

            if level == solution.len() {
                solution.push(node_list[i].down);
            } else if level < solution.len() {
                solution[level] = node_list[i].down;
            } else {
                panic!("Cannot skip levels in solution.");
            }
        } else {
            if !check_exit {
                println!("Visiting solution...");
                for j in 0..level {
                    let mut r = solution[j];
                    while node_list[r].top >= 0 {
                        r += 1;
                    }

                    r = node_list[r].up;
                    let position = item_header[node_list[r].top as usize].name.clone().unwrap();
                    let digit = item_header[node_list[r + 1].top as usize].name.clone().unwrap().chars().last().unwrap();

                    println!("{} {}", position, digit);
                }

                println!("Finished visiting solution!");
            }

            if level == 0 {
                break;
            }

            level -= 1;

            let mut p = solution[level] - 1;

            while p != solution[level] {
                let j = node_list[p].top;
                if j <= 0 {
                    p = node_list[p].down;
                } else {
                    uncover(j as usize, &mut item_header, &mut node_list);
                    p -= 1;
                }
            }

            i = node_list[solution[level]].top as usize;
            solution[level] = node_list[solution[level]].down;
        }

        if solution[level] == i {
            uncover(i, &mut item_header, &mut node_list);
            exit_level = true;
        } else {
            let mut p = solution[level] + 1;

            while p != solution[level] {
                let j = node_list[p].top;
                if j <= 0 {
                    p = node_list[p].up;
                } else {
                    cover(j as usize, &mut item_header, &mut node_list);
                    p += 1;
                }
            }

            level += 1;
        }
    };
}
