//! Dummy NoOp Inspector, helpful as standalone replacemnt.

use crate::Inspector;

#[derive(Clone, Copy)]
pub struct NoOpInspector();

impl<E> Inspector<E> for NoOpInspector {}
