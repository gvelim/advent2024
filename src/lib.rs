pub mod field;
pub mod location;

// id generator fn()
pub fn id_generator(mut start: usize) -> impl FnMut() -> usize {
  move || { start += 1; start }
}
