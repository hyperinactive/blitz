#[macro_export]
macro_rules! output {
    ( $x:expr ) => {{
        print!(">> {}", $x);
    }};
}

#[macro_export]
macro_rules! outputln {
    ( $x:expr ) => {{
        println!(">> {}", $x);
    }};
}
