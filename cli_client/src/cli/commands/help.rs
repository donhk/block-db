pub mod vader_cmds {
    use colored::Colorize;
    use strum::IntoEnumIterator;
    use crate::cli::cmd::CmdTrait;
    use crate::cli::cmds::Cmd;

    pub struct PrintCmd {}

    impl PrintCmd {
        pub fn new() -> PrintCmd {
            PrintCmd {}
        }
    }

    impl CmdTrait for PrintCmd {
        fn execute(&self) {
            for cmd in Cmd::iter() {
                let help = cmd.help();
                println!("{} {}", cmd.emoji(), help.green());
            }
        }
    }
}