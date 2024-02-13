use std::env;
use std::process;

mod easy;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let implementations = vec!["two_sum"];
        println!("Available implementations:");
        for i in implementations.iter() {
            println!("{}", i);
        }
        return;
    }
    match args[1].as_str() {
    "two_sum" => {
        let result = easy::two_sum(vec![2, 7, 11, 15], 9);
        println!("{:?}", result);
    },
    _ => {
        eprintln!("No such implementation: {}", args[1]);
        process::exit(1);
    },
    }
}
