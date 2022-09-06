use crate::sort::Sort;

fn selection_sort(vec: &mut Vec<i32>) {
  let n = vec.len();
  let mut i = 0;
  let mut j;
  let mut k;

  while i < n {
    k = i;
    j = i + 1;
    while j < n {
      if vec[j] < vec[k] {
        k = j
      };
      j = j + 1;
    }
    let temp = vec[k];
    vec[k] = vec[i];
    vec[i] = temp;
    i = i + 1;
  }
}

pub struct SelectionSort {}

impl Sort for SelectionSort {
  fn name(&self) -> &str {
    "selection"
  }
  fn sort(&self, vec: &mut Vec<i32>) {
    selection_sort(vec);
  }
}