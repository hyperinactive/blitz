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

pub fn vec_compare<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}
