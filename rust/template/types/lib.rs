#![allow(
    path_statements,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    unused_parens,
    non_shorthand_field_patterns,
    dead_code,
    overflowing_literals,
    unreachable_patterns,
    unused_variables,
    clippy::unknown_clippy_lints,
    clippy::missing_safety_doc
)]

use num::bigint::BigInt;
use num::FromPrimitive;
use num_traits::identities::One;
use ordered_float::OrderedFloat;
use std::borrow;
use std::fmt;
use std::hash::Hash;
use std::ops::Deref;
use std::os::raw; // TODO: this is  only used by ovn.rs and should be moved there.
use std::result;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;

use lazy_static::lazy_static;

use differential_datalog::ddval::*;
use differential_datalog::decl_enum_into_record;
use differential_datalog::decl_record_mutator_enum;
use differential_datalog::decl_record_mutator_struct;
use differential_datalog::decl_struct_into_record;
use differential_datalog::int::*;
use differential_datalog::program::*;
use differential_datalog::record;
use differential_datalog::record::FromRecord;
use differential_datalog::record::IntoRecord;
use differential_datalog::uint::*;

mod log;
pub use log::*;

/* FlatBuffers code generated by `flatc` */
#[cfg(feature = "flatbuf")]
mod flatbuf_generated;

/* `FromFlatBuffer`, `ToFlatBuffer`, etc, trait declarations. */
#[cfg(feature = "flatbuf")]
pub mod flatbuf;

pub trait Val:
    Eq + Ord + Clone + Hash + PartialEq + PartialOrd + Serialize + DeserializeOwned
{
}

impl<T> Val for T where
    T: Eq + Ord + Clone + Hash + PartialEq + PartialOrd + Serialize + DeserializeOwned
{
}

pub fn string_append_str(mut s1: String, s2: &str) -> String {
    s1.push_str(s2);
    s1
}

#[allow(clippy::ptr_arg)]
pub fn string_append(mut s1: String, s2: &String) -> String {
    s1.push_str(s2.as_str());
    s1
}

#[macro_export]
macro_rules! deserialize_map_from_array {
    ( $modname:ident, $ktype:ty, $vtype:ty, $kfunc:ident ) => {
        mod $modname {
            use super::*;
            use serde::de::{Deserialize, Deserializer};
            use serde::ser::Serializer;
            use std::collections::BTreeMap;

            pub fn serialize<S>(
                map: &__std::std_Map<$ktype, $vtype>,
                serializer: S,
            ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.collect_seq(map.x.values())
            }

            pub fn deserialize<'de, D>(
                deserializer: D,
            ) -> Result<__std::std_Map<$ktype, $vtype>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let v = Vec::<$vtype>::deserialize(deserializer)?;
                Ok(v.into_iter().map(|item| ($kfunc(&item), item)).collect())
            }
        }
    };
}

/*- !!!!!!!!!!!!!!!!!!!! -*/
// Don't edit this line
// Code below this point is needed to test-compile template
// code and is not part of the template.
