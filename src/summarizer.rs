extern crate rustc_serialize;

use std::collections::HashMap;
use rustc_serialize::json::{Json, Object};

pub struct Summarizer {
    count: u64,
    summaries: HashMap<String, Summary>,
}

impl Summarizer {
    pub fn new() -> Summarizer {
        Summarizer{
            count: 0,
            summaries: HashMap::new(),
        }
    }

    pub fn next(&mut self, obj: &Object) {
        self.count += 1;

        for (key, val) in obj.iter() {
            match self.summaries.get_mut(key) {
                Some(summary) => {
                    summary.next(&val);
                    continue;
                },
                None => {},
            };

            let mut summary = Summary::new();
            summary.next(&val);
            self.summaries.insert(key.clone(), summary);
        }
    }

    pub fn num_records(&self) -> u64 {
        self.count
    }

    pub fn results(&self) -> &HashMap<String, Summary> {
        &self.summaries
    }
}

pub struct Summary {
    pub count: u64,
    pub num: NumSummary,
    pub string: StringSummary,
}

impl Summary {
    pub fn new() -> Summary {
        Summary{
            count: 0,
            num: NumSummary::new(),
            string: StringSummary::new(),
        }
    }

    pub fn next(&mut self, val: &Json) {
        self.count += 1;
        match val {
            &Json::I64(num) => self.num.next(num as f64),
            &Json::U64(num) => self.num.next(num as f64),
            &Json::F64(num) => self.num.next(num),
            &Json::String(ref string) => self.string.next(string),
            _ => {},
        }
    }
}

pub struct NumSummary {
    pub count: u64,
    pub min: f64,
    pub max: f64,
    pub sum: f64,
}

impl NumSummary {
    fn new() -> NumSummary {
        NumSummary{
            count: 0u64,
            min: 0f64,
            max: 0f64,
            sum: 0f64,
        }
    }

    fn next(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;

        if self.count == 1 {
            self.min = val;
            self.max = val;
        } else {
            if val < self.min {
                self.min = val;
            }
            if val > self.max {
                self.max = val;
            }
        }
    }

    pub fn avg(&self) -> f64 {
        self.sum / (self.count as f64)
    }
}

pub struct StringSummary {
    pub count: u64,
    pub min_len: usize,
    pub min_word: String,
    pub max_len: usize,
    pub max_word: String,
    pub sum_len: u64,
}

impl StringSummary {
    fn new() -> StringSummary {
        StringSummary{
            count: 0u64,
            min_len: 0,
            min_word: "".to_string(),
            max_len: 0,
            max_word: "".to_string(),
            sum_len: 0u64,
        }
    }

    fn next(&mut self, val: &str) {
        self.count += 1;
        let len = val.len();
        self.sum_len += len as u64;

        if self.count == 1 {
            self.min_len = len;
            self.max_len = len;
        } else {
            if len < self.min_len {
                self.min_len = len;
                self.min_word = val.to_string();
            }
            if len > self.max_len {
                self.max_len = len;
                self.max_word = val.to_string();
            }
        }
    }

    pub fn avg(&self) -> f64 {
        (self.sum_len as f64) / (self.count as f64)
    }
}
