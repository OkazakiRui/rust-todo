use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    // Action::Add -> "buy milk!"
    Add {
        #[structopt()]
        text: String,
    },
    // Action::Done -> task number を受け取り、完了済みにする
    Done {
        #[structopt()]
        position: usize,
    },
    // Action::Done -> タスクをリストで表示する
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-todo",
    about = "command line で使用できる todo アプリケーション"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
    // optionalにする
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
