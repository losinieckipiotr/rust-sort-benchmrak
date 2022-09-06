pub trait Sort {
  fn name(&self) -> &str;
  fn sort(&self, vec: &mut Vec<i32>);
}
