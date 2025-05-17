pub mod field;
pub mod location;

// id generator fn()
pub fn id_generator(mut start: usize) -> impl FnMut() -> usize {
  move || { let tmp = start; start += 1; tmp }
}
