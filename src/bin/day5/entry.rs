use std::fmt::Debug;
use crate::order::Page;

pub(super) struct PrintEntry {
    pub page: Page,
    pub pos: usize
}

impl PartialOrd for PrintEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PrintEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pos.cmp(&other.pos)
    }
}
impl PartialEq for PrintEntry {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}
impl Eq for PrintEntry {

}

impl Debug for PrintEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})",self.page, self.pos)
    }
}
