#[derive(Clone, Copy)]
#[repr(i32)]
pub enum Error {
    Read = -1,
    Write = -2,
    Open = -3,
    Create = -4,
    Conflict = -5,
    Query = -6,
    Parse = -7,
    Compile = -8,
    Format = -9,
}
