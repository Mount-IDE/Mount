#[macro_export]
macro_rules! debug {
    () => {};
    ($str: literal)=> {
        #[cfg(debug_assertions)]
        println!($str)
    };
    ($str: literal, $($x: expr),* ) => {
        #[cfg(debug_assertions)]
        println!($str, $($x), *)
    };
}
