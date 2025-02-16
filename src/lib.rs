mod utils;

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use crate::utils::*;

pub struct DancingLinks {
    item_header: Vec<Record>,
    node_list: Vec<Node>,
    item_index: HashMap<String, usize>,
    backtrack: Vec<usize>,
    primary: usize,
    secondary: usize,
}

impl DancingLinks {
    pub fn new(primary_items: Vec<&str>, secondary_items: Vec<&str>) -> Self {
        let n1 = primary_items.len();
        let n2 = secondary_items.len();
        let n = n1 + n2;

        let mut dlx = DancingLinks {
            item_header: Vec::with_capacity(n + 2),
            node_list: Vec::with_capacity(n + 2),
            item_index: HashMap::new(),
            backtrack: Vec::new(),
            primary: n1,
            secondary: n2,
        };

        dlx.item_header.push(Record::new_unnamed(n1, 1));
        dlx.node_list.push(Node::new_spacer());

        for item in primary_items {
            dlx.add_item(item);
        }

        for item in secondary_items {
            dlx.add_item(item);
        }

        dlx.item_header.push(Record::new_unnamed(n, n1 + 1));
        dlx.node_list.push(Node::new_spacer());

        dlx.item_header[n1].right = 0;
        dlx.item_header[n1 + 1].left = n + 1;

        dlx
    }

    fn add_item(&mut self, item: &str) {
        if self.item_index.contains_key(item) {
            panic!("Item names must be unique.");
        }

        let i = self.get_list_len();

        self.item_header.push(Record::new(item, i));
        self.node_list.push(Node::new_header(i));
        self.item_index.insert(item.to_string(), i);
    }

    pub fn add_option(&mut self, option_str: &str) {
        let mut option_items = HashSet::new();

        let spacer = self.get_list_len() - 1;

        for item_name in option_str.split_whitespace() {
            if let Some(&i) = self.item_index.get(item_name) {
                if option_items.contains(&i) {
                    panic!("Options must contain unique items.");
                }

                let u = self.get_up(i);
                let j = self.get_list_len();

                self.add_node(i);
                self.node_list.push(Node::new(i.try_into().unwrap(), u, i));
                self.set_down(u, j);
                self.set_up(i, j);

                option_items.insert(i);
            } else {
                panic!("Options must contain known items.");
            }
        }

        self.node_list
            .push(Node::new(self.get_top(spacer) - 1, spacer + 1, 0));
        self.set_down(spacer, self.get_list_len() - 2);
    }

    pub fn dance(&mut self) -> (usize, Duration, usize) {
        let now = Instant::now();

        let z = self.get_list_len() - 1;
        self.backtrack
            .resize((-self.get_top(z)).try_into().unwrap(), 0);

        let mut level = 0;

        let mut solution_count = 0;
        let mut visited_nodes = 0;

        let mut exit_level = false;

        loop {
            let check_exit = exit_level;
            exit_level = false;

            let mut i;

            if self.get_right(0) != 0 && !check_exit {
                visited_nodes += 1;

                let mut min_length = usize::MAX;
                let mut p = self.get_right(0);
                i = p;

                while p != 0 {
                    let length = self.get_length(p);

                    if length < min_length {
                        min_length = length;
                        i = p;

                        if min_length == 0 {
                            break;
                        }
                    }

                    p = self.get_right(p);
                }

                self.cover(i);

                self.backtrack[level] = self.get_down(i);
            } else {
                if !check_exit {
                    visited_nodes += 1;

                    solution_count += 1;

                    println!("Solution {solution_count}:");

                    for l in 0..level {
                        let option_str = self.get_option_str(self.backtrack[l]);

                        println!("\t{option_str}");
                    }
                }

                if level == 0 {
                    break;
                }

                level -= 1;

                let x = self.backtrack[level];
                let mut p = x - 1;

                while p != x {
                    let j = self.get_top(p);
                    if j <= 0 {
                        p = self.get_down(p);
                    } else {
                        self.uncover(j.try_into().unwrap());
                        p -= 1;
                    }
                }

                i = self.get_top(x).try_into().unwrap();
                self.backtrack[level] = self.get_down(x);
            }

            if self.backtrack[level] == i {
                self.uncover(i);
                exit_level = true;
            } else {
                let x = self.backtrack[level];
                let mut p = x + 1;

                while p != x {
                    let j = self.get_top(p);
                    if j <= 0 {
                        p = self.get_up(p);
                    } else {
                        self.cover(j.try_into().unwrap());
                        p += 1;
                    }
                }

                level += 1;
            }
        }

        (solution_count, now.elapsed(), visited_nodes)
    }

    fn cover(&mut self, i: usize) {
        let mut p = self.get_down(i);

        while p != i {
            self.hide(p);
            p = self.get_down(p);
        }

        let l = self.get_left(i);
        let r = self.get_right(i);

        self.set_right(l, r);
        self.set_left(r, l);
    }

    fn hide(&mut self, p: usize) {
        let mut q = p + 1;

        while q != p {
            let t = self.get_top(q);
            let u = self.get_up(q);
            let d = self.get_down(q);

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

    fn uncover(&mut self, i: usize) {
        let l = self.get_left(i);
        let r = self.get_right(i);

        self.set_right(l, i);
        self.set_left(r, i);

        let mut p = self.get_up(i);

        while p != i {
            self.unhide(p);
            p = self.get_up(p);
        }
    }

    fn unhide(&mut self, p: usize) {
        let mut q = p - 1;

        while q != p {
            let t = self.get_top(q);
            let u = self.get_up(q);
            let d = self.get_down(q);

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

    pub fn get_primary(&self) -> usize {
        self.primary
    }

    pub fn get_secondary(&self) -> usize {
        self.secondary
    }

    pub fn get_item_count(&self) -> usize {
        self.primary + self.secondary
    }

    pub fn get_option_count(&self) -> usize {
        (-self.get_top(self.get_list_len() - 1)).try_into().unwrap()
    }

    fn get_list_len(&self) -> usize {
        self.node_list.len()
    }

    fn get_option_str(&self, i: usize) -> String {
        let t = self.get_top(i);

        if t <= 0 {
            panic!("Node {i} does not correspond to an item in an option.");
        }

        let mut option_str = self.item_header[t as usize].name.clone().unwrap();

        let mut p = i + 1;

        while p != i {
            let t = self.get_top(p);

            if t <= 0 {
                p = self.get_up(p);
                continue;
            }

            let name = " ".to_owned() + &self.item_header[t as usize].name.clone().unwrap();

            option_str.push_str(&name);

            p += 1;
        }

        option_str
    }

    fn get_length(&self, i: usize) -> usize {
        self.item_header[i].length
    }

    fn get_left(&self, i: usize) -> usize {
        self.item_header[i].left
    }

    fn get_right(&self, i: usize) -> usize {
        self.item_header[i].right
    }

    fn get_top(&self, i: usize) -> isize {
        self.node_list[i].top
    }

    fn get_up(&self, i: usize) -> usize {
        self.node_list[i].up
    }

    fn get_down(&self, i: usize) -> usize {
        self.node_list[i].down
    }

    fn set_left(&mut self, i: usize, l: usize) {
        self.item_header[i].left = l;
    }

    fn set_right(&mut self, i: usize, r: usize) {
        self.item_header[i].right = r;
    }

    fn set_up(&mut self, i: usize, u: usize) {
        self.node_list[i].up = u;
    }

    fn set_down(&mut self, i: usize, d: usize) {
        self.node_list[i].down = d;
    }

    fn add_node(&mut self, i: usize) {
        self.item_header[i].add_node();
    }

    fn remove_node(&mut self, i: usize) {
        self.item_header[i].remove_node();
    }
}
