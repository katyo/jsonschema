/*!

[BSON](http://bsonspec.org/) format (binary)

*/

pub fn from_slice<T>(s: &[u8]) -> bson::de::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut s = std::io::Cursor::new(s);
    let doc = bson::Document::from_reader(&mut s)?;
    bson::from_document(doc)
}
