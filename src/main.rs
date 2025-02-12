use std::io;

use dlx::DancingLinks;

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

    let mut dlx = DancingLinks::new(primary_items, secondary_items);

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

        dlx.add_option(&option_buffer);

        option_buffer.clear();
    };

    let mut level = 0;
    let mut solution = Vec::new();
    let mut exit_level = false;

    loop {
        let check_exit = exit_level;
        exit_level = false;

        let mut i;

        if dlx.get_root_item().right != 0 && !check_exit {
            let mut min_length = usize::MAX;
            let mut p = dlx.get_root_item().right;
            i = p;

            while p != 0 {
                let length = dlx.get_item(p).length;

                if length < min_length {
                    min_length = length;
                    i = p;

                    if min_length == 0 {
                        break;
                    }
                }

                p = dlx.get_item(p).right;
            }

            dlx.cover(i);

            if level == solution.len() {
                solution.push(dlx.get_node(i).down);
            } else if level < solution.len() {
                solution[level] = dlx.get_node(i).down;
            } else {
                panic!("Cannot skip levels in solution.");
            }
        } else {
            if !check_exit {
                println!("Visiting solution...");
                for j in 0..level {
                    let mut r = solution[j];
                    while dlx.get_node(r).top >= 0 {
                        r += 1;
                    }

                    r = dlx.get_node(r).up;
                    let position = dlx.get_item(dlx.get_node(r).top.try_into().unwrap()).name.clone().unwrap();
                    let digit = dlx.get_item(dlx.get_node(r + 1).top.try_into().unwrap()).name.clone().unwrap().chars().last().unwrap();

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
                let j = dlx.get_node(p).top;
                if j <= 0 {
                    p = dlx.get_node(p).down;
                } else {
                    dlx.uncover(j.try_into().unwrap());
                    p -= 1;
                }
            }

            i = dlx.get_node(solution[level]).top.try_into().unwrap();
            solution[level] = dlx.get_node(solution[level]).down;
        }

        if solution[level] == i {
            dlx.uncover(i);
            exit_level = true;
        } else {
            let mut p = solution[level] + 1;

            while p != solution[level] {
                let j = dlx.get_node(p).top;
                if j <= 0 {
                    p = dlx.get_node(p).up;
                } else {
                    dlx.cover(j.try_into().unwrap());
                    p += 1;
                }
            }

            level += 1;
        }
    };
}
