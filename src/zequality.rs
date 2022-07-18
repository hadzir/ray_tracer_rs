use num_traits::Float;

pub trait ZEq<T>
{
    fn zeq(&self, other: T) -> bool;

    fn zneg(&self, other: T) -> bool {
        !self.zeq(other)
    }
}

impl<T> ZEq<T> for T 
where
    T: Float,
{
    fn zeq(&self, other: T) -> bool {
        return (*self - other).abs() < T::from(0.00001).unwrap();
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
