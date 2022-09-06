use crate::sort::Sort;

pub struct DefaultSort {}

impl Sort for DefaultSort {
  fn name(&self) -> &str {
    "Default"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    vec.sort();
  }
}
