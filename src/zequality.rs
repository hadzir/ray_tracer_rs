use crate::F;

pub trait ZEq<T>
{
    fn zeq(&self, other: T) -> bool;

    fn zneg(&self, other: T) -> bool {
        !self.zeq(other)
    }
}

impl ZEq<F> for F 
{
    fn zeq(&self, other: F) -> bool {
        return (*self - other).abs() < F::from(0.00001);
    }
}
impl<T> ZEq<Option<T>> for Option<T>
where
  T: Clone,
  T: ZEq<T>,
{
  fn zeq(&self, other: Option<T>) -> bool {
    match (self, other) {
      (Some(ref option), Some(other)) => option.zeq(other),
      (None, None) => true,
      _ => false,
    }
  }
}
//Might need to be revisited
#[macro_export]
macro_rules! assert_zeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zneg(*right) {
                    panic!(
                        "asserting zequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! assert_nzeq {
    ($left:expr,$right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left, right) => {
                if left.zeq(*right) {
                    panic!(
                        "asserting inzequality between {:?} and {:?} failed",
                        left, right
                    )
                }
            }
        }
    }};
}
