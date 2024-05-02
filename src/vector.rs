use anyhow::{anyhow, Result};
use std::ops::{Deref, Index};
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Mul},
};

// 点积数据的结构体
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Vector { data: data.into() }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// 实现解引用，返回Vec<T>
impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// 做点积 得到一个标量
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Debug + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow!("Vector A length must be equal to Vector B length"));
    }
    let mut result = T::default();
    for (x, y) in a.iter().zip(b.iter()) {
        result += *x * *y;
    }
    Ok(result)
}
