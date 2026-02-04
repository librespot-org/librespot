use std::{fmt::Debug, ops::RangeInclusive, str::FromStr};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum ParseFileSizeError {
    #[error("empty argument")]
    EmptyInput,
    #[error("invalid suffix")]
    InvalidSuffix,
    #[error("invalid number: {0}")]
    InvalidNumber(#[from] std::num::ParseFloatError),
    #[error("non-finite number specified")]
    NotFinite(f64),
}

pub fn parse_file_size(input: &str) -> Result<u64, ParseFileSizeError> {
    use ParseFileSizeError::*;

    let mut iter = input.chars();
    let mut suffix = iter.next_back().ok_or(EmptyInput)?;
    let mut suffix_len = 0;

    let iec = matches!(suffix, 'i' | 'I');

    if iec {
        suffix_len += 1;
        suffix = iter.next_back().ok_or(InvalidSuffix)?;
    }

    let base: u64 = if iec { 1024 } else { 1000 };

    suffix_len += 1;
    let exponent = match suffix.to_ascii_uppercase() {
        '0'..='9' if !iec => {
            suffix_len -= 1;
            0
        }
        'K' => 1,
        'M' => 2,
        'G' => 3,
        'T' => 4,
        'P' => 5,
        'E' => 6,
        'Z' => 7,
        'Y' => 8,
        _ => return Err(InvalidSuffix),
    };

    let num = {
        let mut iter = input.chars();

        for _ in (&mut iter).rev().take(suffix_len) {}

        iter.as_str().parse::<f64>()?
    };

    if !num.is_finite() {
        return Err(NotFinite(num));
    }

    Ok((num * base.pow(exponent) as f64) as u64)
}

#[derive(Debug, Error)]
pub enum ParserFactoryError<T> {
    #[error("invalid number: {0}")]
    InvalidFloat(#[from] std::num::ParseFloatError),
    #[error("invalid number: {0}")]
    InvalidInt(#[from] std::num::ParseIntError),
    #[error("Value must lay between {0} and {1}")]
    NotInRange(T, T),
}

pub fn range_parser_factory<T>(
    range: RangeInclusive<T>,
) -> impl Fn(&str) -> Result<T, ParserFactoryError<T>> + Clone + Send + Sync + 'static
where
    T: FromStr + PartialOrd + Copy + Send + Sync + 'static,
    ParserFactoryError<T>: From<<T as FromStr>::Err>,
{
    move |value: &str| {
        let num_value = value.parse::<T>()?;
        if !range.contains(&num_value) {
            Err(ParserFactoryError::NotInRange(*range.start(), *range.end()))
        } else {
            Ok(num_value)
        }
    }
}

#[derive(Debug, Error)]
pub enum ProxyParserError {
    #[error("invalid number: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("Invalid proxy url, only URLs on the format \"http(s)://host:port\" are allowed")]
    InvalidProxyUrl,
}

pub fn proxy_parser(value: &str) -> Result<Url, ProxyParserError> {
    let url = Url::parse(value)?;
    if url.host().is_none() || url.port_or_known_default().is_none() {
        Err(ProxyParserError::InvalidProxyUrl)
    } else {
        Ok(url)
    }
}
