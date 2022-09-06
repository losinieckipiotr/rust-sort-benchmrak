use crate::sort::Sort;

fn insertion_sort(vec: &mut Vec<i32>) {
  let mut key;
  let mut j;
  let len = vec.len();

  let mut i = 1;
  while i < len {
    key = vec[i];
    j = i;

    loop {
      if j > 0 && vec[j - 1] > key {
        vec[j] = vec[j - 1];
        j = j - 1;
      } else {
        break;
      }
    }
    vec[j] = key;
    i = i + 1;
  }
}

pub struct InsertionSort {}

impl Sort for InsertionSort {
  fn name(&self) -> &str {
    "insertion"
  }

  fn sort(&self, vec: &mut Vec<i32>) {
    insertion_sort(vec);
  }
}
