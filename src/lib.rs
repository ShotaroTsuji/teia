pub mod z2vector;
pub mod chaincomplex;

pub trait Index : Ord + Copy {}

impl Index for u64 {}
impl Index for u32 {}
impl Index for usize {}
