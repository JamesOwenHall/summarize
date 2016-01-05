extern crate rustc_serialize;

mod summarizer;

use std::io::BufRead;
use summarizer::Summarizer;
use rustc_serialize::json::Json;

fn main() {
    let mut summarizer = Summarizer::new();
    let stdin = std::io::stdin();
    let values = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| Json::from_str(&line).ok());
    
    for value in values {
        let object = match value.as_object() {
            Some(object) => object,
            None => continue,
        };
        summarizer.next(&object);
    }

    println!("Total number of records: {}", summarizer.num_records());
    for (key, summary) in summarizer.results() {
        println!("\nField \"{}\" (count: {})", key, summary.count);
        println!("=================");

        if summary.null_count > 0 {
            println!("Null => count: {}", summary.null_count);
        }

        if summary.obj_count > 0 {
            println!("Object => count: {}", summary.obj_count);
        }

        if summary.num.count > 0 {
            let num = &summary.num;
            println!(
                "Number => count: {}, min: {}, max: {}, avg: {:.*}",
                num.count, num.min, num.max, 4, num.avg(),
            );
        }

        if summary.string.count > 0 {
            let string = &summary.string;
            println!(
                "String => count: {}, shortest: {}, longest: {}, avg length: {:.*}",
                string.count, string.min_word, string.max_word, 4, string.avg(),
            );
        }

        if summary.boolean.count > 0 {
            let boolean = &summary.boolean;
            println!(
                "Boolean => count: {}, # of false: {}, # of true: {}",
                boolean.count, boolean.num_false, boolean.num_true,
            );
        }

        if summary.array.count > 0 {
            let array = &summary.array;
            println!(
                "Array => count: {}, shortest: {}, longest: {}, avg length: {:.*}",
                array.count, array.min, array.max, 4, array.avg(),
            );
        }
    }
}
