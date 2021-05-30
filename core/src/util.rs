use std::mem;

pub trait Seq {
    fn next(&self) -> Self;
}

macro_rules! impl_seq {
    ($($ty:ty)*) => { $(
        impl Seq for $ty {
            fn next(&self) -> Self { (*self).wrapping_add(1) }
        }
    )* }
}

impl_seq!(u8 u16 u32 u64 usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct SeqGenerator<T: Seq>(T);

impl<T: Seq> SeqGenerator<T> {
    pub fn new(value: T) -> Self {
        SeqGenerator(value)
    }

    pub fn get(&mut self) -> T {
        let value = self.0.next();
        mem::replace(&mut self.0, value)
    }
}

pub trait PacketData: Copy {
    fn write(self, vec: &mut Vec<u8>);

    fn size_hint(&self) -> usize;
}

pub trait IntoPacketData<T = Self> {
    type Data: PacketData;

    fn convert(data: T) -> Self::Data;
}

// `()` represents an empty packet
impl PacketData for () {
    #[inline]
    fn size_hint(&self) -> usize {
        0
    }

    #[inline]
    fn write(self, _: &mut Vec<u8>) {}
}

// `(A, B)` represents the A and B concatenated
impl<A, B> PacketData for (A, B)
where
    A: PacketData,
    B: PacketData,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.size_hint() + self.1.size_hint()
    }

    #[inline]
    fn write(self, vec: &mut Vec<u8>) {
        self.0.write(vec);
        self.1.write(vec);
    }
}

macro_rules! impl_packet_data_num {
    ( $($t:ty : $n:literal), *) => {
        $(
            impl PacketData for $t {
                #[inline]
                fn size_hint(&self) -> usize {
                    $n
                }

                #[inline]
                fn write(self, vec: &mut Vec<u8>) {
                    let bytes : [u8; $n] = self.to_be_bytes();
                    vec.extend_from_slice(&bytes);
                }
            }

            impl IntoPacketData for $t {
                type Data = Self;

                #[inline]
                fn convert(s: Self) -> Self {
                    s
                }
            }
        )*
    };
}

impl_packet_data_num!(
    u8: 1, u16: 2, u32: 4, u64: 8, u128: 16,
    i8: 1, i16: 2, i32: 4, i64: 8, i128: 16
);

impl PacketData for &[u8] {
    #[inline]
    fn size_hint(&self) -> usize {
        self.len()
    }

    #[inline]
    fn write(self, vec: &mut Vec<u8>) {
        vec.extend_from_slice(self)
    }
}

impl<'a, T: ?Sized> IntoPacketData<&'a T> for T
where
    T: AsRef<[u8]>,
{
    type Data = &'a [u8];

    #[inline]
    fn convert(t: &'a T) -> &'a [u8] {
        t.as_ref()
    }
}

pub struct ProtoPacketData<'a, T>(&'a T);

impl<'a, T> Clone for ProtoPacketData<'a, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<'a, T> Copy for ProtoPacketData<'a, T> {}

impl<'a, T> PacketData for ProtoPacketData<'a, T>
where
    T: protobuf::Message,
{
    #[inline]
    fn size_hint(&self) -> usize {
        self.0.compute_size() as usize
    }

    #[inline]
    fn write(self, vec: &mut Vec<u8>) {
        self.0.write_to_vec(vec).unwrap();
    }
}

pub enum Proto {}

impl<'a, T> IntoPacketData<&'a T> for Proto
where
    T: protobuf::Message,
{
    type Data = ProtoPacketData<'a, T>;

    #[inline]
    fn convert(data: &'a T) -> Self::Data {
        ProtoPacketData(data)
    }
}

pub struct PacketBuilder<T: PacketData>(T);

impl Default for PacketBuilder<()> {
    #[inline]
    fn default() -> Self {
        Self(())
    }
}

impl PacketBuilder<()> {
    #[inline]
    pub fn new() -> Self {
        Self(())
    }
}

impl<T: PacketData> PacketBuilder<T> {
    #[inline]
    pub fn append<U: PacketData>(self, other: U) -> PacketBuilder<(T, U)> {
        PacketBuilder((self.0, other))
    }

    #[inline]
    pub fn build(self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(self.0.size_hint());
        self.0.write(&mut vec);
        vec
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[macro_export]
macro_rules! packet {
    ($(($t:ty) $e:expr),*) => {
        $crate::util::PacketBuilder::new()
        $(
            .append(<$t as $crate::util::IntoPacketData<_>>::convert($e))
        )*
        .build()
    };
}
