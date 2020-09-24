/*!

[JSON5](https://json5.org/) format (text)

*/

pub use json5::from_str;

pub fn from_slice<'a, T>(s: &'a [u8]) -> json5::Result<T>
where
    T: serde::Deserialize<'a>,
{
    let s = std::str::from_utf8(s).map_err(|error| json5::Error::Message(error.to_string()))?;
    from_str(s)
}
