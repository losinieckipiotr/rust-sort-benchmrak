use  crate::utils::swap;

pub fn bubble_sort(vec: &mut Vec<i32>) {
  let n = vec.len();
  let mut j;

  for i in 0..n {
    j = 0;
    while j < n - 1 - i {
      if vec[j] > vec[j + 1] {
        // vec.swap(j, j + 1);
        swap(j, j + 1, vec);
      }
      j = j + 1;
    }
  }
}
