pub mod vader_cmds {
    use crate::cli::cmd::CmdTrait;
    use colored::*;

    pub struct UnknownCmd<'a> {
        raw_cmd: &'a str,
    }

    impl<'a> UnknownCmd<'a> {
        pub fn new(raw_cmd: &'a str) -> Self {
            UnknownCmd {
                raw_cmd
            }
        }
    }

    impl<'a> CmdTrait for UnknownCmd<'a> {
        fn execute(&self) {
            let bomb = emojis::get("💣").unwrap();
            println!("{} Unknown command '{}'", bomb.as_str(), self.raw_cmd.red());
        }
    }
}