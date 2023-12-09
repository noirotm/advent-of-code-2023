use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub struct WhitespaceSeparatedList<T>(Vec<T>);

impl<T> AsRef<[T]> for WhitespaceSeparatedList<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> From<WhitespaceSeparatedList<T>> for Vec<T> {
    fn from(value: WhitespaceSeparatedList<T>) -> Self {
        value.0
    }
}

impl<T> FromStr for WhitespaceSeparatedList<T>
where
    T: FromStr,
{
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_ascii_whitespace()
            .map(T::from_str)
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v))
    }
}

pub trait ReadExt<T> {
    fn split_by<B: FromIterator<T>>(self, separator: u8) -> B;
    fn split_commas<B: FromIterator<T>>(self) -> B;
    fn split_lines<B: FromIterator<T>>(self) -> B;
    fn split_groups<B: FromIterator<T>>(self) -> B;
}

impl<R, T> ReadExt<T> for R
where
    R: Read,
    T: FromStr,
{
    fn split_by<B: FromIterator<T>>(self, separator: u8) -> B {
        BufReader::new(self)
            .split(separator)
            .flatten()
            .flat_map(String::from_utf8)
            .flat_map(|s| s.parse())
            .collect()
    }

    fn split_commas<B: FromIterator<T>>(self) -> B {
        self.split_by(b',')
    }

    fn split_lines<B: FromIterator<T>>(self) -> B {
        BufReader::new(self)
            .lines()
            .flatten()
            .flat_map(|l| l.parse())
            .collect()
    }

    fn split_groups<B: FromIterator<T>>(self) -> B {
        BufReader::new(self)
            .lines()
            .flatten()
            .collect::<Vec<_>>()
            .split(|l| l.is_empty())
            .flat_map(|e| e.join("\n").parse())
            .collect()
    }
}
