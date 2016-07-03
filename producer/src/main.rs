use std::env;

fn main() {

    if env::args().nth(1).is_none() {
        println!("Argument [number] is missing");
        return;
    }

    let count = env::args()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .expect("Argument must be an integer");

    for _ in 0..(count*10) {
        for i in 0..(count / 2) {
            println!("https://aws.amazon.com/articles/Elastic-MapReduce/{}", i);
        }

        for i in (0..(count / 2)).rev() {
            println!("https://aws.amazon.com/articles/Elastic-MapReduce/{}", i);
        }
    }
}
