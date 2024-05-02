use std::{
    fmt::Display,
    sync::mpsc::{self, Sender},
    thread,
};

const SUM: u8 = 10;
fn main() -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..SUM {
        let tx: Sender<Message> = tx.clone();
        thread::spawn(move || handle_message(i, tx));
    }

    drop(tx); // Drop tx to close the channel

    let handle = thread::spawn(move || {
        let mut size: u128 = 0;
        for received in rx {
            size += received.data as u128;
            println!("Received: {:?}", received);
        }
        size
    });

    let data = handle
        .join()
        .map_err(|e| anyhow::anyhow!("Thread error: {:?}", e))?;

    println!("Data: {}", data);
    Ok(())
}

fn handle_message(data: u8, tx: Sender<Message>) -> anyhow::Result<()> {
    loop {
        tx.send(Message::new(data))?;
        let random = rand::random::<u8>();
        if random % 5 == 0 {
            println!("Thread {} is done", data);
            break;
        }
        thread::sleep(std::time::Duration::from_millis(random as u64 * 10));
    }

    Ok(())
}

#[warn(dead_code)]
#[derive(Debug)]
struct Message {
    thread: u8,
    data: u8,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Thread: {}, Data: {}", self.thread, self.data)
    }
}

impl Message {
    fn new(thread: u8) -> Self {
        let data: u8 = rand::random();
        Message { thread, data }
    }
}
