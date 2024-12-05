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

/// Parse file into vector of `T`
pub fn file_to_vec<T: FromStr>(path: &str, sep: &str) -> Result<Vec<T>> {
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

/// Parse file into matrix (vector of vectors) of `T`
pub fn file_to_matrix<T: FromStr>(path: &str, sep_v: &str, sep_h: &str) -> Result<Vec<Vec<T>>> {
    Ok(fs::read_to_string(path)?
        .split(sep_v)
        .filter_map(|line| line
            .is_empty()
            .not()
            .then(|| line
                .trim()
                .split(sep_h)
                .filter_map(|s| s
                    .is_empty()
                    .not()
                    .then(|| s
                        .trim()
                        .parse()
                        .ok())?)
                .collect()))
        .collect())
}
