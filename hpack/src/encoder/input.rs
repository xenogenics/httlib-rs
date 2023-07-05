/// Provides encoder input format options.
#[derive(Debug)]
pub enum EncoderInput<'a> {
    /// Represents a fully indexed header field.
    Indexed(u32),

    /// Represents a header field where name is represented by an index and the
    /// value is provided in bytes. This format can hold configuration flags.
    IndexedNameOwned(u32, Vec<u8>, u8),
    IndexedNameBorrowed(u32, &'a [u8], u8),

    /// Represents a header field where name and value are provided in bytes.
    /// This format can hold configuration flags.
    LiteralOwned(Vec<u8>, Vec<u8>, u8),
    LiteralBorrowed(&'a [u8], &'a [u8], u8),
}

impl<'a> From<u32> for EncoderInput<'a> {
    fn from(field: u32) -> Self {
        EncoderInput::Indexed(field)
    }
}

impl<'a> From<(u32, Vec<u8>, u8)> for EncoderInput<'a> {
    fn from(field: (u32, Vec<u8>, u8)) -> Self {
        EncoderInput::IndexedNameOwned(field.0, field.1, field.2)
    }
}

impl<'a> From<(u32, &'a [u8], u8)> for EncoderInput<'a> {
    fn from(field: (u32, &'a [u8], u8)) -> Self {
        EncoderInput::IndexedNameBorrowed(field.0, field.1, field.2)
    }
}

impl<'a> From<(Vec<u8>, Vec<u8>, u8)> for EncoderInput<'a> {
    fn from(field: (Vec<u8>, Vec<u8>, u8)) -> Self {
        EncoderInput::LiteralOwned(field.0, field.1, field.2)
    }
}

impl<'a> From<(&'a [u8], &'a [u8], u8)> for EncoderInput<'a> {
    fn from(field: (&'a [u8], &'a [u8], u8)) -> Self {
        EncoderInput::LiteralBorrowed(field.0, field.1, field.2)
    }
}
