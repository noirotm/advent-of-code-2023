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
