use std::{
    fs,
    str::FromStr,
    error::Error,
};

#[macro_export]
macro_rules! flush {
    () => {
        use std::io::Write;
        std::io::stdout().flush().expect("Unable to flush `stdout`");
    }
}

#[macro_export]
macro_rules! read {
    ($input: ident) => {
        std::io::stdin().read_line(&mut $input).expect("Unable to read from `stdin`");
    }
}

// inspired by teej_dv's implementation
pub fn read_from_file<T: FromStr>(path: &str, sep: &str) -> Result<Vec<T>, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?
        .split(sep)
        .filter_map(|line| if !line.is_empty() {
            line.trim().parse().ok()
        } else {
            None
        })
        .collect())
}
