use clap::Parser;

use devutils::utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name("number"))]
    number: String,
}

fn main() {
    let args = Args::parse();

    let value = utils::parse_number(&args.number);

    if let Some(value) = value {
        println!("{}", utils::number_in_base(value, 8));
    } else {
        println!("[ERROR]: Couldn't parse number!");
    };
}
