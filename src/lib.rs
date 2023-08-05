use std::{
    fs,
    io::Result,
    ops::Not,
    str::FromStr,
};

#[macro_export]
macro_rules! flush {
    () => {
        std::io::Write::flush(&mut std::io::stdout()).expect("Unable to flush `stdout`");
    }
}

#[macro_export]
macro_rules! read {
    ($input: ident) => {
        std::io::stdin().read_line(&mut $input).expect("Unable to read from `stdin`");
    }
}

// Inspired by teej_dv's implementation
pub fn read_from_file<T: FromStr>(path: &str, sep: &str) -> Result<Vec<T>> {
    Ok(fs::read_to_string(path)?
        .split(sep)
        .filter_map(|line| line
            .is_empty()
            .not()
            .then(|| line
                .trim()
                .parse()
                .ok())?)
        .collect())
}
