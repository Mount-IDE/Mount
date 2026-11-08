
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

#[macro_export]
macro_rules! on_debug {
    () => {};
    ($($x: tt)*)=> {
        #[cfg(any(debug_assertions, test))]
        {
            $($x) *
        }
        #[cfg(not(any(debug_assertions, test)))]
        {
            ()
        }
    }
}
