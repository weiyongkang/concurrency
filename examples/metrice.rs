use std::thread;

use anyhow::Result;
use concurrency::Metrice;
use rand::Rng;

const N: u8 = 2;
const M: u8 = 4;

fn main() -> Result<()> {
    let metrice = Metrice::new();

    println!("{}", metrice);

    for i in 0..N {
        task_worker(i.into(), metrice.clone())?;
    }

    for _ in 0..M {
        request_worker(metrice.clone())?;
    }

    loop {
        thread::sleep(std::time::Duration::from_secs(2));
        println!("{}", metrice);
    }

    // Ok(())
}

fn task_worker(inx: usize, mut metrice: Metrice) -> Result<()> {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        thread::sleep(std::time::Duration::from_millis(rng.gen_range(100..5000)));
        metrice.inc(format!("call.thread.worker.{}", inx))?;
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(mut metrice: Metrice) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(std::time::Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..10);
            metrice.inc(format!("req.page.{}", page))?;
        }

        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
