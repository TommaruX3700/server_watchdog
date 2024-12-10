pub trait Plugin {
    fn name(&self) -> &'static str;
    fn execute(&self);
}
