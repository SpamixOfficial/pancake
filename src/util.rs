#[macro_export]
macro_rules! exists {
    ($x:expr) => {
        $x.try_exists().map_or(false, |x| x)
    };
}