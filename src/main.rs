mod cli;
use structopt::StructOpt;

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
    ////// cargo run add "buy milk!"
    // CommandLineArgs {
    //     action: Add {
    //         text: "buy milk",
    //     },
    //     journal_file: None,
    // }

    ////// cargo run done 12
    // CommandLineArgs {
    //     action: Done {
    //         position: 12,
    //     },
    //     journal_file: None,
    // }

    ////// cargo run list
    // CommandLineArgs {
    //     action: List,
    //     journal_file: None,
    // }

    ////// cargo run -- -j sample.txt list
    // CommandLineArgs {
    //     action: List,
    //     journal_file: Some(
    //         "sample.txt",
    //     ),
    // }
}
