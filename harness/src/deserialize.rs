#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct DeserFromStr<T: FromStr + Eq>(#[serde(deserialize_with = "from_str")] T)
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
