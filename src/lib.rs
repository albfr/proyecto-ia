pub mod config;

mod utils;

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use crate::config::*;
use crate::utils::*;

pub struct DancingLinks {
    item_header: Vec<Record>,
    node_list: Vec<Node>,
    item_index: HashMap<String, usize>,
    primary: usize,
    secondary: usize,
}

impl DancingLinks {
    pub fn new(primary_items: &[&str], secondary_items: &[&str]) -> Self {
        let n1 = primary_items.len();
        let n2 = secondary_items.len();
        let n = n1 + n2;

        let mut dlx = DancingLinks {
            item_header: Vec::with_capacity(n + 2),
            node_list: Vec::with_capacity(n + 2),
            item_index: HashMap::new(),
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
        self.item_index.insert(String::from(item), i);
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

    pub fn dance(&mut self, config: &Config) -> (usize, Duration, usize, usize, usize, usize) {
        let now = Instant::now();

        let z = self.get_list_len() - 1;
        let mut backtrack = vec![0; (-self.get_top(z)).try_into().unwrap()];

        let mut level = 0;
        let mut exit_level = false;
        let mut i;

        let mut solution_count = 0;
        let mut visited_nodes = 0;
        let mut update_count = 0;
        let mut max_degree = 0;
        let mut max_level = 0;

        let timeout = config.get_timeout().map(Duration::from_secs);

        let report_delta = Duration::from_secs(config.get_report_delta());
        let mut time_threshold = report_delta;

        let level_limit = 3 * config.get_level_limit();

        let show_first = config.show_first();
        let solution_interval = config.get_solution_interval();

        loop {
            let time_elapsed = now.elapsed();

            if let Some(t) = timeout {
                if time_elapsed >= t {
                    println!("TIMEOUT!");

                    return (
                        solution_count,
                        time_elapsed,
                        visited_nodes,
                        update_count,
                        max_degree,
                        max_level,
                    );
                }
            }

            if time_elapsed >= time_threshold {
                let mut branches = String::new();

                let mut explored = 0.0;
                let mut d = 1.0;

                for &x in backtrack.iter().take(level) {
                    let (position, length) = self.get_option_position(x);

                    let length_char =
                        char::from_digit(length.try_into().unwrap(), 36).unwrap_or('*');

                    d *= length as f64;

                    if let Some(k) = position {
                        let position_char =
                            char::from_digit(k.try_into().unwrap(), 36).unwrap_or('*');

                        branches.push_str(&format!("{}{} ", position_char, length_char));

                        explored += ((k - 1) as f64) / d;
                    } else {
                        branches.push_str(&format!("?{} ", length_char));
                    }
                }

                let elapsed = time_elapsed.as_secs();

                if branches.len() > level_limit {
                    branches = branches.chars().take(level_limit).collect::<String>();
                    branches.push_str("...");
                } else if !branches.is_empty() {
                    branches.pop();
                }

                let s = if solution_count == 1 {
                    "solution"
                } else {
                    "solutions"
                };

                if level_limit == 0 {
                    eprintln!(
                        "{}s: {} {}, {:.5} explored",
                        elapsed, solution_count, s, explored,
                    );
                } else {
                    eprintln!(
                        "{}s: {} {}, {}, {:.5} explored",
                        elapsed, solution_count, s, branches, explored,
                    );
                }

                time_threshold += report_delta;
            }

            let check_exit = exit_level;
            exit_level = false;

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

                if max_degree < min_length {
                    max_degree = min_length;
                }

                update_count += self.cover(i);

                backtrack[level] = self.get_down(i);
            } else {
                if !check_exit {
                    visited_nodes += 1;

                    if max_level < level + 1 {
                        max_level = level + 1;
                    }

                    solution_count += 1;

                    if (show_first && solution_count == 1)
                        || (solution_interval > 0 && solution_count % solution_interval == 0)
                    {
                        println!("Solution {}:", solution_count);

                        for &x in backtrack.iter().take(level) {
                            println!(" {}", self.get_option_str(x));
                        }
                    }
                }

                if level == 0 {
                    break;
                }

                level -= 1;

                let x = backtrack[level];
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
                backtrack[level] = self.get_down(x);
            }

            if backtrack[level] == i {
                self.uncover(i);
                exit_level = true;
            } else {
                let x = backtrack[level];
                let mut p = x + 1;

                while p != x {
                    let j = self.get_top(p);
                    if j <= 0 {
                        p = self.get_up(p);
                    } else {
                        update_count += self.cover(j.try_into().unwrap());
                        p += 1;
                    }
                }

                level += 1;

                if max_level < level {
                    max_level = level;
                }
            }
        }

        (
            solution_count,
            now.elapsed(),
            visited_nodes,
            update_count,
            max_degree,
            max_level,
        )
    }

    fn cover(&mut self, i: usize) -> usize {
        let mut updates = 1;

        let mut p = self.get_down(i);

        while p != i {
            updates += self.hide(p);
            p = self.get_down(p);
        }

        let l = self.get_left(i);
        let r = self.get_right(i);

        self.set_right(l, r);
        self.set_left(r, l);

        updates
    }

    fn hide(&mut self, p: usize) -> usize {
        let mut updates = 0;

        let mut q = p + 1;

        while q != p {
            let t = self.get_top(q);
            let u = self.get_up(q);
            let d = self.get_down(q);

            if t <= 0 {
                q = u;
            } else {
                updates += 1;

                self.set_down(u, d);
                self.set_up(d, u);
                self.remove_node(t.try_into().unwrap());
                q += 1;
            }
        }

        updates
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

    fn get_option_position(&self, i: usize) -> (Option<usize>, usize) {
        let t = self.get_top(i);

        if t <= 0 {
            panic!("Node {i} does not correspond to an item in an option.");
        }

        let t: usize = t.try_into().unwrap();

        let mut p = self.get_down(t);

        let mut k = 1;

        while p != i && p != t {
            p = self.get_down(p);

            k += 1;
        }

        let length = self.get_length(t);

        if p != t {
            (Some(k), length)
        } else {
            (None, length)
        }
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

            option_str.push_str(&format!(
                " {}",
                self.item_header[t as usize].name.clone().unwrap()
            ));

            p += 1;
        }

        let (position, length) = self.get_option_position(i);

        if let Some(k) = position {
            option_str.push_str(&format!(" ({} of {})", k, length));
        } else {
            option_str.push_str(&format!(" (? of {})", length));
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
