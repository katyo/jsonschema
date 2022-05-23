/*!

[Pickle](https://docs.python.org/3/library/pickle.html) format (binary)

*/

pub fn from_slice<'de, T: serde::Deserialize<'de>>(
    data: &[u8],
) -> pickle::Result<T> {
    pickle::from_slice(data, pickle::de::DeOptions::default())
}
