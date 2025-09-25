// Problem:
// Write a function called apply_to_lengths that:
// Takes a Vec<String> and a closure.
// Calls the closure on each string in the vector.
// Returns a Vec containing whatever the closure produces for each string.
// Then, in main, call apply_to_lengths with:
// A vector of words: ["apple", "banana", "pear"].
// A closure that returns the length of each word.

// Expected output: [5, 6, 4]

// Hints:
// Your function will need generics: one for the input element type, one for the output type, and one for the closure type.
// The closure will be called once per string.
// The return type is a Vec of whatever the closure returns.
pub fn run() {
    let input = vec![1,2,3,4,5];
    let new_vec = apply_to_lengths(input, |item| item.to_string())
}

pub fn apply_to_lengths<T, K, F>(input: Vec<T>, f: F) -> Vec<K>
where
    F: FnMut(&T) -> K,
    {
        let mut output = Vec::new();
        for item in input.iter() {
            output.push(f(item));
        }
        output
    }