use std::{
    borrow::{Borrow, BorrowMut},
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Deserializer};

fn from_str<'de, T: FromStr, D: Deserializer<'de>>(deser: D) -> Result<T, D::Error>
where
    T::Err: Debug,
{
    use serde::de::Error;
    let s: std::borrow::Cow<'_, str> = Deserialize::deserialize(deser)?;
    s.parse()
        .map_err(|err| D::Error::custom(format!("{err:?}")))
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct DeserFromStr<T: FromStr + Eq>(#[serde(deserialize_with = "from_str")] pub T)
where
    T::Err: Debug;
impl<T: Hash + FromStr + Eq> Hash for DeserFromStr<T>
where
    T::Err: Debug,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
impl<T: FromStr + Eq> Deref for DeserFromStr<T>
where
    T::Err: Debug,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: FromStr + Eq> DerefMut for DeserFromStr<T>
where
    T::Err: Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: FromStr + Eq> Borrow<T> for DeserFromStr<T>
where
    T::Err: Debug,
{
    fn borrow(&self) -> &T {
        &self.0
    }
}
impl<T: FromStr + Eq> BorrowMut<T> for DeserFromStr<T>
where
    T::Err: Debug,
{
    fn borrow_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
