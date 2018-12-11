#[macro_export]
macro_rules! stdlines {
    () => {{
        use std::io;
        use std::io::BufRead;
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>()
    }}
}
