use std::collections::hash_state::HashState;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::path;
use std::rc::Rc;
use std::str;
use std::sync::Arc;

///////////////////////////////////////////////////////////////////////////////

pub trait Serialize {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer;
}

///////////////////////////////////////////////////////////////////////////////

pub trait Serializer {
    type Error;

    fn visit_bool(&mut self, v: bool) -> Result<(), Self::Error>;

    #[inline]
    fn visit_isize(&mut self, v: isize) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i8(&mut self, v: i8) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i16(&mut self, v: i16) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i32(&mut self, v: i32) -> Result<(), Self::Error> {
        self.visit_i64(v as i64)
    }

    #[inline]
    fn visit_i64(&mut self, v: i64) -> Result<(), Self::Error>;

    #[inline]
    fn visit_usize(&mut self, v: usize) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u8(&mut self, v: u8) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u16(&mut self, v: u16) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u32(&mut self, v: u32) -> Result<(), Self::Error> {
        self.visit_u64(v as u64)
    }

    #[inline]
    fn visit_u64(&mut self, v: u64) -> Result<(), Self::Error>;

    #[inline]
    fn visit_f32(&mut self, v: f32) -> Result<(), Self::Error> {
        self.visit_f64(v as f64)
    }

    fn visit_f64(&mut self, v: f64) -> Result<(), Self::Error>;

    #[inline]
    fn visit_char(&mut self, v: char) -> Result<(), Self::Error> {
        // The unwraps in here should be safe.
        let mut s = &mut [0; 4];
        let len = v.encode_utf8(s).unwrap();
        self.visit_str(str::from_utf8(&s[..len]).unwrap())
    }

    fn visit_str(&mut self, value: &str) -> Result<(), Self::Error>;

    fn visit_unit(&mut self) -> Result<(), Self::Error>;

    #[inline]
    fn visit_named_unit(&mut self, _name: &str) -> Result<(), Self::Error> {
        self.visit_unit()
    }

    #[inline]
    fn visit_enum_unit(&mut self,
                       _name: &str,
                       _variant: &str) -> Result<(), Self::Error> {
        self.visit_unit()
    }

    fn visit_none(&mut self) -> Result<(), Self::Error>;

    fn visit_some<V>(&mut self, value: V) -> Result<(), Self::Error>
        where V: Serialize;

    fn visit_seq<V>(&mut self, visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor;

    #[inline]
    fn visit_named_seq<V>(&mut self,
                          _name: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor,
    {
        self.visit_seq(visitor)
    }

    #[inline]
    fn visit_enum_seq<V>(&mut self,
                         _name: &'static str,
                         _variant: &'static str,
                         visitor: V) -> Result<(), Self::Error>
        where V: SeqVisitor,
    {
        self.visit_seq(visitor)
    }

    fn visit_seq_elt<T>(&mut self,
                        first: bool,
                        value: T) -> Result<(), Self::Error>
        where T: Serialize;

    fn visit_map<V>(&mut self, visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor;

    #[inline]
    fn visit_named_map<V>(&mut self,
                          _name: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor,
    {
        self.visit_map(visitor)
    }

    #[inline]
    fn visit_enum_map<V>(&mut self,
                          _name: &'static str,
                          _variant: &'static str,
                          visitor: V) -> Result<(), Self::Error>
        where V: MapVisitor,
    {
        self.visit_map(visitor)
    }

    fn visit_map_elt<K, V>(&mut self,
                           first: bool,
                           key: K,
                           value: V) -> Result<(), Self::Error>
        where K: Serialize,
              V: Serialize;
}

pub trait SeqVisitor {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer;

    /// Return the length of the sequence if known.
    #[inline]
    fn len(&self) -> Option<usize> {
        None
    }
}

pub trait MapVisitor {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer;

    /// Return the length of the map if known.
    #[inline]
    fn len(&self) -> Option<usize> {
        None
    }
}

///////////////////////////////////////////////////////////////////////////////

macro_rules! impl_visit {
    ($ty:ty, $method:ident) => {
        impl Serialize for $ty {
            #[inline]
            fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
                where S: Serializer,
            {
                serializer.$method(*self)
            }
        }
    }
}

impl_visit!(bool, visit_bool);
impl_visit!(isize, visit_isize);
impl_visit!(i8, visit_i8);
impl_visit!(i16, visit_i16);
impl_visit!(i32, visit_i32);
impl_visit!(i64, visit_i64);
impl_visit!(usize, visit_usize);
impl_visit!(u8, visit_u8);
impl_visit!(u16, visit_u16);
impl_visit!(u32, visit_u32);
impl_visit!(u64, visit_u64);
impl_visit!(f32, visit_f32);
impl_visit!(f64, visit_f64);
impl_visit!(char, visit_char);

///////////////////////////////////////////////////////////////////////////////

impl<'a> Serialize for &'a str {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_str(*self)
    }
}

impl Serialize for String {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (&self[..]).serialize(serializer)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<T> Serialize for Option<T> where T: Serialize {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        match *self {
            Some(ref value) => serializer.visit_some(value),
            None => serializer.visit_none(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct SeqIteratorVisitor<Iter> {
    iter: Iter,
    len: Option<usize>,
    first: bool,
}

impl<T, Iter> SeqIteratorVisitor<Iter>
    where Iter: Iterator<Item=T>
{
    #[inline]
    pub fn new(iter: Iter, len: Option<usize>) -> SeqIteratorVisitor<Iter> {
        SeqIteratorVisitor {
            iter: iter,
            len: len,
            first: true,
        }
    }
}

impl<T, Iter> SeqVisitor for SeqIteratorVisitor<Iter>
    where T: Serialize,
          Iter: Iterator<Item=T>,
{
    #[inline]
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer,
    {
        let first = self.first;
        self.first = false;

        match self.iter.next() {
            Some(value) => {
                let value = try!(serializer.visit_seq_elt(first, value));
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn len(&self) -> Option<usize> {
        self.len
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<'a, T> Serialize for &'a [T]
    where T: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_seq(SeqIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

impl<T> Serialize for Vec<T> where T: Serialize {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (&self[..]).serialize(serializer)
    }
}

impl<T> Serialize for BTreeSet<T>
    where T: Serialize + Ord,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_seq(SeqIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

impl<T, H> Serialize for HashSet<T, H>
    where T: Serialize + Eq + Hash,
          H: HashState,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_seq(SeqIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Serialize for () {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_unit()
    }
}

///////////////////////////////////////////////////////////////////////////////

// FIXME(rust #19630) Remove this work-around
macro_rules! e {
    ($e:expr) => { $e }
}

macro_rules! tuple_impls {
    ($(
        $TupleVisitor:ident ($len:expr, $($T:ident),+) {
            $($state:pat => $idx:tt,)+
        }
    )+) => {
        $(
            pub struct $TupleVisitor<'a, $($T: 'a),+> {
                tuple: &'a ($($T,)+),
                state: u8,
                first: bool,
            }

            impl<'a, $($T: 'a),+> $TupleVisitor<'a, $($T),+> {
                pub fn new(tuple: &'a ($($T,)+)) -> $TupleVisitor<'a, $($T),+> {
                    $TupleVisitor {
                        tuple: tuple,
                        state: 0,
                        first: true,
                    }
                }
            }

            impl<'a, $($T),+> SeqVisitor for $TupleVisitor<'a, $($T),+>
                where $($T: Serialize),+
            {
                fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
                    where S: Serializer,
                {
                    let first = self.first;
                    self.first = false;

                    match self.state {
                        $(
                            $state => {
                                self.state += 1;
                                Ok(Some(try!(serializer.visit_seq_elt(first, &e!(self.tuple.$idx)))))
                            }
                        )+
                        _ => {
                            Ok(None)
                        }
                    }
                }

                fn len(&self) -> Option<usize> {
                    Some($len)
                }
            }

            impl<$($T),+> Serialize for ($($T,)+)
                where $($T: Serialize),+
            {
                #[inline]
                fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
                    serializer.visit_seq($TupleVisitor::new(self))
                }
            }
        )+
    }
}

tuple_impls! {
    TupleVisitor1 (1, T0) {
        0 => 0,
    }
    TupleVisitor2 (2, T0, T1) {
        0 => 0,
        1 => 1,
    }
    TupleVisitor3 (3, T0, T1, T2) {
        0 => 0,
        1 => 1,
        2 => 2,
    }
    TupleVisitor4 (4, T0, T1, T2, T3) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
    }
    TupleVisitor5 (5, T0, T1, T2, T3, T4) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
    }
    TupleVisitor6 (6, T0, T1, T2, T3, T4, T5) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
    }
    TupleVisitor7 (7, T0, T1, T2, T3, T4, T5, T6) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
    }
    TupleVisitor8 (8, T0, T1, T2, T3, T4, T5, T6, T7) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
    }
    TupleVisitor9 (9, T0, T1, T2, T3, T4, T5, T6, T7, T8) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
    }
    TupleVisitor10 (10, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
    }
    TupleVisitor11 (11, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        10 => 10,
    }
    TupleVisitor12 (12, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 7,
        8 => 8,
        9 => 9,
        10 => 10,
        11 => 11,
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct MapIteratorVisitor<Iter> {
    iter: Iter,
    len: Option<usize>,
    first: bool,
}

impl<K, V, Iter> MapIteratorVisitor<Iter>
    where Iter: Iterator<Item=(K, V)>
{
    #[inline]
    pub fn new(iter: Iter, len: Option<usize>) -> MapIteratorVisitor<Iter> {
        MapIteratorVisitor {
            iter: iter,
            len: len,
            first: true,
        }
    }
}

impl<K, V, I> MapVisitor for MapIteratorVisitor<I>
    where K: Serialize,
          V: Serialize,
          I: Iterator<Item=(K, V)>,
{
    #[inline]
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error>
        where S: Serializer,
    {
        let first = self.first;
        self.first = false;

        match self.iter.next() {
            Some((key, value)) => {
                let value = try!(serializer.visit_map_elt(first, key, value));
                Ok(Some(value))
            }
            None => Ok(None)
        }
    }

    #[inline]
    fn len(&self) -> Option<usize> {
        self.len
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<K, V> Serialize for BTreeMap<K, V>
    where K: Serialize + Ord,
          V: Serialize,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_map(MapIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

impl<K, V, H> Serialize for HashMap<K, V, H>
    where K: Serialize + Eq + Hash,
          V: Serialize,
          H: HashState,
{
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        serializer.visit_map(MapIteratorVisitor::new(self.iter(), Some(self.len())))
    }
}

///////////////////////////////////////////////////////////////////////////////

impl<'a, T> Serialize for &'a T where T: Serialize {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<'a, T> Serialize for &'a mut T where T: Serialize {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<T> Serialize for Box<T> where T: Serialize {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<T> Serialize for Rc<T> where T: Serialize, {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

impl<T> Serialize for Arc<T> where T: Serialize, {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        (**self).serialize(serializer)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Serialize for path::Path {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        self.to_str().unwrap().serialize(serializer)
    }
}

impl Serialize for path::PathBuf {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: Serializer,
    {
        self.to_str().unwrap().serialize(serializer)
    }
}
