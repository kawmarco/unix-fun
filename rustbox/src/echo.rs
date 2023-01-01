// Simple `echo` command, prints args separated by space

pub fn main(args: Vec<String>) {
    println!("{}", args.join(" "));
}
