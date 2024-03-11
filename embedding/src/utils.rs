use std::fmt::Debug;

pub fn batch<T>(inputs: &Vec<T>, batch_size: i64) -> Vec<Vec<T>>
where
    T: Copy + Debug,
{
    let mut results: Vec<Vec<T>> = Vec::new();
    let mut batch: Vec<T> = Vec::new();

    for (i, input) in inputs.iter().enumerate() {
        if i as i64 % batch_size == 0 && i != 0 {
            results.push(batch);
            batch = Vec::new();
        }
        batch.push(input.clone());
    }

    if (results.len() == 0) || (batch.len() > 0) {
        results.push(batch);
    }

    results
}
