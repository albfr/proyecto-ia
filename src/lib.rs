mod utils;

use std::collections::{HashMap, HashSet};

use crate::utils::*;

pub struct DancingLinks {
    item_header: Vec<Record>,
    node_list: Vec<Node>,
    item_index: HashMap<String, usize>,
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

                let u = self.get_node(i).up;
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
            .push(Node::new(self.get_node(spacer).top - 1, spacer + 1, 0));
        self.set_down(spacer, self.get_list_len() - 2);
    }

    pub fn dance(&mut self) {
        let mut level = 0;
        let mut solution = Vec::new();
        let mut exit_level = false;

        let mut total_solutions = 0;

        loop {
            let check_exit = exit_level;
            exit_level = false;

            let mut i;

            if self.get_item(0).right != 0 && !check_exit {
                let mut min_length = usize::MAX;
                let mut p = self.get_item(0).right;
                i = p;

                while p != 0 {
                    let length = self.get_item(p).length;

                    if length < min_length {
                        min_length = length;
                        i = p;

                        if min_length == 0 {
                            break;
                        }
                    }

                    p = self.get_item(p).right;
                }

                self.cover(i);

                if level == solution.len() {
                    solution.push(self.get_node(i).down);
                } else if level < solution.len() {
                    solution[level] = self.get_node(i).down;
                } else {
                    panic!("Cannot skip levels in solution.");
                }
            } else {
                if !check_exit {
                    total_solutions += 1;

                    if total_solutions % 1000000 == 0 {
                        println!("found another million!");
                        println!("total so far: {total_solutions}");
                    }
                    /*
                    println!("Visiting solution...");
                    for j in 0..level {
                        let mut r = solution[j];
                        while self.get_node(r).top >= 0 {
                            r += 1;
                        }

                        r = self.get_node(r).up;
                        let position = self.get_item(self.get_node(r).top.try_into().unwrap()).name.clone().unwrap();
                        let digit = self.get_item(self.get_node(r + 1).top.try_into().unwrap()).name.clone().unwrap().chars().last().unwrap();

                        println!("{} {}", position, digit);
                    }

                    println!("Finished visiting solution!");
                    */
                }

                if level == 0 {
                    break;
                }

                level -= 1;

                let mut p = solution[level] - 1;

                while p != solution[level] {
                    let j = self.get_node(p).top;
                    if j <= 0 {
                        p = self.get_node(p).down;
                    } else {
                        self.uncover(j.try_into().unwrap());
                        p -= 1;
                    }
                }

                i = self.get_node(solution[level]).top.try_into().unwrap();
                solution[level] = self.get_node(solution[level]).down;
            }

            if solution[level] == i {
                self.uncover(i);
                exit_level = true;
            } else {
                let mut p = solution[level] + 1;

                while p != solution[level] {
                    let j = self.get_node(p).top;
                    if j <= 0 {
                        p = self.get_node(p).up;
                    } else {
                        self.cover(j.try_into().unwrap());
                        p += 1;
                    }
                }

                level += 1;
            }
        }

        println!("Finished dancing. Found {total_solutions} solutions.");
    }

    fn cover(&mut self, i: usize) {
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

    fn uncover(&mut self, i: usize) {
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

    fn get_item(&self, i: usize) -> &Record {
        &self.item_header[i]
    }

    fn get_node(&self, i: usize) -> &Node {
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
}
