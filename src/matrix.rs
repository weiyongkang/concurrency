use anyhow::{anyhow, Result};
use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread, vec,
};

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

// #[derive(Debug)]
pub struct Matrix<T: Debug> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

// 实现矩阵的乘法, 重载 * 运算符, 返回一个新的矩阵
impl<T> Mul for Matrix<T>
where
    T: Debug + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Send + 'static,
{
    type Output = Self;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        multiply(&self, &rhs).expect("Error multiplying matrices")
    }
}

// 定义一个输入的结构体，包含行，列，以及索引
pub struct MsgInput<T> {
    inx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(inx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        MsgInput { inx, row, col }
    }
}

// 定义一个消息结构体，包含输入和发送者
impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Msg { input, sender }
    }
}

// 定义一个输出的结构体，包含索引和结果
pub struct MsgOutput<T> {
    inx: usize,
    result: T,
}

// 定义一个消息结构体，包含输入和发送者
pub struct Msg<T> {
    input: MsgInput<T>,
    // sender to send the result
    sender: oneshot::Sender<MsgOutput<T>>,
}

// 矩阵的乘法
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Send + 'static,
{
    if a.cols != b.rows {
        return Err(anyhow!("Matrix A columns must be equal to Matrix B rows"));
    }

    // 初始化 线程 ，定义 channel 为 mpsc 模式
    // 发送者为 tx，接收者为 rx，发送者发送 Msg<T> 类型的消息，接收者接收 MsgOutput<T> 类型的消息
    // 接收数据获取点积数据后，在通过接收到的对象msg 中的 sender 发送数据
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || -> Result<()> {
                // Modify the closure to return a Result type
                for msg in rx {
                    let result = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        inx: msg.input.inx,
                        result,
                    }) {
                        eprintln!("Error: {}", e);
                    };
                }
                Ok::<_, anyhow::Error>(()) // Return Ok(()) at the end of the closure
            });
            tx
        })
        .collect::<Vec<_>>();

    // generate 4 threads
    //
    let matrix_len = a.rows * b.cols;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.rows {
        for j in 0..b.cols {
            let row = Vector::new(&a.data[i * a.cols..(i + 1) * a.cols]);
            // 二维数组 获得列的数据，需要跳过 b.cols 个元素
            let col_data = b.data[j..]
                .iter()
                .step_by(b.cols) // 跳过 b.cols 个元素 获取下一个元素
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col_data);
            let inx = i * b.cols + j;
            let input = MsgInput::new(inx, row, col);

            // 创建一个 oneshot channel，发送者为 tx，接收者为 rx，发送者设置为 Msg的 sender，接收者 添加到 receivers 中
            let (tx, rx) = oneshot::channel::<MsgOutput<T>>();
            let msg = Msg::new(input, tx);
            senders[inx % NUM_THREADS]
                .send(msg)
                .map_err(|op| anyhow!(op.to_string()))?;
            receivers.push(rx);
        }
    }

    // 等待所有的线程结束, 并将结果保存到 data 中
    for rx in receivers {
        let output = rx.recv().map_err(|op| anyhow!(op.to_string()))?;
        data[output.inx] = output.result;
    }

    Ok(Matrix {
        data,
        rows: a.rows,
        cols: b.cols,
    })
}

impl<T> Matrix<T>
where
    T: Debug,
{
    pub fn new(data: impl Into<Vec<T>>, rows: usize, cols: usize) -> Self {
        Matrix {
            data: data.into(),
            rows,
            cols,
        }
    }
}

impl<T> Display for Matrix<T>
where
    T: Debug,
{
    // display the matrix a 2x3 as {1 2 3, 4 5 6}, 3x2 as {1 2, 3 4, 5 6}
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{:?}", self.data[i * self.cols + j])?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }
            if i < self.rows - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")
    }
}

impl<T> Debug for Matrix<T>
where
    T: Display + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matrix(rows={},cols={},{})", self.rows, self.cols, self)
    }
}

impl<T> Default for Matrix<T>
where
    T: Default + Debug + Copy,
{
    fn default() -> Self {
        Matrix {
            data: vec![T::default(); 0],
            rows: 0,
            cols: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(format!("{:?}", c), "Matrix(rows=2,cols=2,{22 28, 49 64})");
        Ok(())
    }

    #[test]
    fn test_matrix_error() -> Result<()> {
        let a = Matrix::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3, 3);
        let b = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_ne!(format!("{:?}", c), "Matrix(rows=2,cols=2,{22 28, 49 64})");
        Ok(())
    }
}
