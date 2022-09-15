pub mod bubble;
pub mod selection;
pub mod insertion;
pub mod quick;
pub mod quick2;
pub mod quick3;
pub mod default;
pub mod merge;

pub use bubble::bubble_sort;
pub use selection::selection_sort;
pub use insertion::insertion_sort;
pub use quick::quick_sort;
pub use quick2::quick_sort_2;
pub use quick3::quick_sort_3;
pub use default::default_sort;
pub use default::default_unstable_sort;
pub use merge::merge_sort;
