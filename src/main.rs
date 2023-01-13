use clap::Parser;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
