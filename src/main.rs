use ll_lexer::lexer;

mod compute;
mod parse;
mod rule_table;
mod symbol;

use compute::compute;
use parse::parse;
use rule_table::get_rt;
use symbol::get_symbol;

fn get_arg() -> String {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("usage:\texpr");
        std::process::exit(1);
    }

    return args.remove(1);
}

fn main() {
    let lexed = lexer(get_arg(), get_rt(), &get_symbol);
    let parsed = parse(&lexed.unwrap()).unwrap();

    println!("{:?}", compute(&parsed));
}
