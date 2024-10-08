use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::{mapper, reducer};

pub fn map_reduce_str(input: Vec<String>) -> HashMap<String, usize> {
    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for line in input {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mapped = mapper::map(&line);
            results_clone.lock().unwrap().push(mapped);
        });
        handles.push(handle);
    }

    println!("Finish Map");
    /////////////////////////////////////////////////

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finish Shuffling");
    /////////////////////////////////////////////////

    let mut final_result = HashMap::new();
    for mapped_data in results.lock().unwrap().iter() {
        let reduced = reducer::reduce(mapped_data.clone());
        for (word, count) in reduced {
            *final_result.entry(word).or_insert(0) += count;
        }
    }

    println!("Finish Reduce");
    /////////////////////////////////////////////////

    final_result
}
