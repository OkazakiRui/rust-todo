use std::path::PathBuf;

pub enum Action {
    // Action::Add -> "buy milk!"
    Add { task: String },
    // Action::Done -> task number を受け取り、完了済みにする
    Done { position: usize },
    // Action::Done -> タスクをリストで表示する
    List,
}

pub struct CommandLineArgs {
    pub action: Action,
    // optionalにする
    pub journal_file: Option<PathBuf>,
}
