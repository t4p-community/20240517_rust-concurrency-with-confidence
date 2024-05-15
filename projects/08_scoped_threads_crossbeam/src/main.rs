// import the crossbeam::thread module, which provides tools for scoped threading.
use crossbeam::thread;

fn main() {
    // create a mutable vector of integers.
    let mut numbers = vec![1, 2, 3, 4, 5];

    // create a scoped thread environment using crossbeam's thread::scope function.
    thread::scope(|s| {
        // iterate over mutable references to each element in the numbers vector.
        for num in &mut numbers {
            // spawn a new thread within the scoped environment.
            s.spawn(move |_| {
                // increment the current number by 10.
                *num += 10;
                // print the updated number.
                println!("Number: {}", num);
            });
        }

        // observe that handles are not being collected here
        // because the threads are spawned within the scoped environment
        // and will be joined automatically when the scope ends
    })
    // handle any potential errors from the scoped thread environment.
    .unwrap();

    // print the numbers vector after all modifications.
    println!("Numbers after modification: {:?}", numbers);
}
