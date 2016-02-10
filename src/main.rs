extern crate rustc_serialize;

mod summarizer;

use std::io::BufRead;
use summarizer::Summarizer;
use rustc_serialize::json::Json;

fn main() {
    let mut summarizer = Summarizer::new();
    let stdin = std::io::stdin();
    let objects = stdin
        .lock()
        .lines()
        .enumerate()
        .map(|tup| {
            let (line_num, line) = tup;
            line.map_err(|_| format!("Can't read line {} from stdin.", line_num))
                .and_then(|line| Json::from_str(&line).map_err(|_| format!("Can't parse JSON on line {}.", line_num)))
                .and_then(|json| json.as_object().cloned().ok_or(format!("JSON value on line {} is not an object.", line_num)))
        });

    for object in objects {
        match object {
            Ok(obj) => summarizer.next(&obj),
            Err(err) => {
                println!("{:}", err);
                return
            },
        }
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
