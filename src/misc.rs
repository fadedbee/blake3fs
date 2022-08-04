use std::convert::AsMut;

pub fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut array = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut array).clone_from_slice(slice);
    array
}