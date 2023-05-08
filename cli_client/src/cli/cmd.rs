
pub trait CmdTrait {
    fn execute(&self, raw_cmd: &str);
}