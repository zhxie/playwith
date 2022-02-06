use structopt::StructOpt;

use playwith as lib;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Parse arguments
    let flags = Flags::from_args();

    // Log
    lib::set_logger(flags.verbose);
}

#[derive(StructOpt, Clone, Debug, Eq, Hash, PartialEq)]
#[structopt(about)]
struct Flags {
    #[structopt(
        long,
        short,
        help = "Prints verbose information (-vv for vverbose)",
        parse(from_occurrences)
    )]
    pub verbose: usize,
}
