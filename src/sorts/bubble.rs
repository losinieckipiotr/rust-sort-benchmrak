use crate::sort::Sort;

pub fn bubble_sort(vec: &mut Vec<i32>) {
  let n = vec.len();
  let mut temp;
  let mut j;

  for i in 0..n {
    j = 0;
    while j < n - 1 - i {
      if vec[j] > vec[j + 1] {
        temp = vec[j + 1];
        vec[j + 1] = vec[j];
        vec[j] = temp;
      }
      j = j + 1;
    }
  }
}

pub struct BubbleSort {}

impl Sort for BubbleSort {
  fn name(&self) -> &str {
    "bubble"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    bubble_sort(vec);
  }
}
