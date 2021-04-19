use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Write tasks to the journal file.
    Add {
        /// The task description text.
        #[structopt()]
        text: String,
    },
    /// Remove an entry from the journal file by position.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "TODO List Practice", about = "A Rust TODO_List CLI")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a different journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}

#[cfg(test)]
mod test {
    use super::Action;
    use super::CommandLineArgs;
    use std::path::PathBuf;
    use structopt::StructOpt;

    #[test]
    fn test_from_args() {
        CommandLineArgs::from_args();
    }
}
