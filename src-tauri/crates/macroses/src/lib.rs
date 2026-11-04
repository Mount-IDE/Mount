#[macro_export]
macro_rules! log {
    ($($x:tt) *) => {

        #[cfg(any(debug_assertions, test))]
        {
            println!("_________________");
            println!("| File `{}` at `{}` line:\n{:?}", file!(), line!(), format_args!($($x) *));
            println!("_________________");
        }
    };
    ()=>{

    };
}
