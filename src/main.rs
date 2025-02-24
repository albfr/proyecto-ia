use std::env;
use std::io;
use std::process;
use std::time::Instant;

use dlx::config::*;
use dlx::DancingLinks;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(args.as_slice()).unwrap_or_else(|err| {
        panic!("{}", err);
    });

    if config.help() {
        config.show_help();

        process::exit(0);
    }

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

    let mut dlx = DancingLinks::new(&config, primary_items.as_slice(), secondary_items.as_slice());

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
    }

    let preprocess_time = now.elapsed();

    eprintln!(
        "Read {} entries: {}+{}={} items and {} options.",
        (dlx.get_list_len() - 1) - (dlx.get_item_count() + 1),
        dlx.get_primary(),
        dlx.get_secondary(),
        dlx.get_item_count(),
        dlx.get_option_count(),
    );

    let (solution_count, elapsed_time, visited_nodes, update_count, max_degree, max_level) = dlx.dance();

    let s = if solution_count == 1 { "" } else { "s" };

    if config.is_verbose() {
        let total_time = preprocess_time + elapsed_time;

        println!(
            "The tree's maximum degree is {}, its depth is {}.",
            max_degree, max_level,
        );

        println!(
            "{:.5?} overall: {:.5?} processing input + {:.5?} dancing.",
            total_time, preprocess_time, elapsed_time,
        );

        println!(
            "{:.5?} per solution.",
            elapsed_time / (solution_count.try_into().unwrap())
        );
    }

    println!(
        "Found {} solution{} in {:.5?} visiting {} nodes doing {} updates.",
        solution_count, s, elapsed_time, visited_nodes, update_count,
    );
}
