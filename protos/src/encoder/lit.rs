/// Provides encoder input format options.
///
/// This is a list of all binary formats supported by the encoder.
#[derive(Debug)]
pub enum EncoderLit<'a> {
    /// Represents `binary` format of wire type `2`.
    Bytes(&'a Vec<u8>),

    /// Represents `bool` format of wire type `0`.
    Bool(&'a bool),

    /// Represents `bool` format of wire type `2` for packed repeated fields.
    BoolVec(&'a Vec<bool>),

    /// Represents `int32` format of wire type `0`.
    Int32(&'a i32),

    /// Represents `int32` format of wire type `0` for packed repeated fields.
    Int32Vec(&'a Vec<i32>),

    /// Represents `int64` format of wire type `0`.
    Int64(&'a i64),

    /// Represents `int64` format of wire type `0` for packed repeated fields.
    Int64Vec(&'a Vec<i64>),

    /// Represents `uint32` format of wire type `0`.
    UInt32(&'a u32),

    /// Represents `uint32` format of wire type `0` for packed repeated fields.
    UInt32Vec(&'a Vec<u32>),

    /// Represents `uint64` format of wire type `0`.
    UInt64(&'a u64),

    /// Represents `uint64` format of wire type `0` for packed repeated fields.
    UInt64Vec(&'a Vec<u64>),

    /// Represents `float` format of wire type `5`.
    Float(&'a f32),

    /// Represents `float` format of wire type `5` for packed repeated fields.
    FloatVec(&'a Vec<f32>),

    /// Represents `uint32` format of wire type `1`.
    Double(&'a f64),

    /// Represents `double` format of wire type `1` for packed repeated fields.
    DoubleVec(&'a Vec<f64>),

    /// Represents `sint32` format of wire type `0`. Use it when the value is
    /// likely to be negative.
    SInt32(&'a i32),

    /// Represents `sint32` format of wire type `0` for packed repeated fields.
    /// Use it when the values are likely to be negative.
    SInt32Vec(&'a Vec<i32>),

    /// Represents `sint64` format of wire type `0`. Use it when the value is
    /// likely to be negative.
    SInt64(&'a i64),

    /// Represents `sint64` format of wire type `0` for packed repeated fields.
    /// Use it when the values are likely to be negative.
    SInt64Vec(&'a Vec<i64>),

    /// Represents `fixed32` format of wire type `5`.
    Fixed32(&'a u32),

    /// Represents `fixed32` format of wire type `5` for packed repeated fields.
    Fixed32Vec(&'a Vec<u32>),

    /// Represents `fixed64` format of wire type `1`.
    Fixed64(&'a u64),

    /// Represents `fixed64` format of wire type `1` for packed repeated fields.
    Fixed64Vec(&'a Vec<u64>),

    /// Represents `sfixed32` format of wire type `5`.
    SFixed32(&'a i32),

    /// Represents `sfixed32` format of wire type `5` for packed repeated
    /// fields.
    SFixed32Vec(&'a Vec<i32>),

    /// Represents `sfixed64` format of wire type `1`.
    SFixed64(&'a i64),

    /// Represents `sfixed64` format of wire type `1` for packed repeated
    /// fields.
    SFixed64Vec(&'a Vec<i64>),
}

impl<'a> From<&'a bool> for EncoderLit<'a> {
    fn from(v: &'a bool) -> Self {
        Self::Bool(v)
    }
}

impl<'a> From<&'a Vec<bool>> for EncoderLit<'a> {
    fn from(v: &'a Vec<bool>) -> Self {
        Self::BoolVec(v)
    }
}

impl<'a> From<&'a i32> for EncoderLit<'a> {
    fn from(v: &'a i32) -> Self {
        Self::Int32(v)
    }
}

impl<'a> From<&'a Vec<i32>> for EncoderLit<'a> {
    fn from(v: &'a Vec<i32>) -> Self {
        Self::Int32Vec(v)
    }
}

impl<'a> From<&'a i64> for EncoderLit<'a> {
    fn from(v: &'a i64) -> Self {
        Self::Int64(v)
    }
}

impl<'a> From<&'a Vec<i64>> for EncoderLit<'a> {
    fn from(v: &'a Vec<i64>) -> Self {
        Self::Int64Vec(v)
    }
}

impl<'a> From<&'a u32> for EncoderLit<'a> {
    fn from(v: &'a u32) -> Self {
        Self::UInt32(v)
    }
}

impl<'a> From<&'a Vec<u32>> for EncoderLit<'a> {
    fn from(v: &'a Vec<u32>) -> Self {
        Self::UInt32Vec(v)
    }
}

impl<'a> From<&'a u64> for EncoderLit<'a> {
    fn from(v: &'a u64) -> Self {
        Self::UInt64(v)
    }
}

impl<'a> From<&'a Vec<u64>> for EncoderLit<'a> {
    fn from(v: &'a Vec<u64>) -> Self {
        Self::UInt64Vec(v)
    }
}

impl<'a> From<&'a f32> for EncoderLit<'a> {
    fn from(v: &'a f32) -> Self {
        Self::Float(v)
    }
}

impl<'a> From<&'a Vec<f32>> for EncoderLit<'a> {
    fn from(v: &'a Vec<f32>) -> Self {
        Self::FloatVec(v)
    }
}

impl<'a> From<&'a f64> for EncoderLit<'a> {
    fn from(v: &'a f64) -> Self {
        Self::Double(v)
    }
}

impl<'a> From<&'a Vec<f64>> for EncoderLit<'a> {
    fn from(v: &'a Vec<f64>) -> Self {
        Self::DoubleVec(v)
    }
}

impl<'a> From<&'a Vec<u8>> for EncoderLit<'a> {
    fn from(v: &'a Vec<u8>) -> Self {
        Self::Bytes(v)
    }
}
