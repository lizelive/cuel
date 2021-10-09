use std::{collections::HashMap, sync::Arc};

use crate::id::Id;

trait Fabric<T, M> {
    fn sub(&self, key: &Id) -> Self;
    fn get(&self) -> T;
    fn set(&mut self, value: T);
    fn push(&mut self, msg: M);
    fn pop(&mut self) -> Option<T>;
}

struct MemFabric<T, M> {
    value: T,
    messages: Vec<M>,
}

impl<T, M> MemFabric<T, M> {}

