use super::{ThreadMessage, ThreadPool};

use crossbeam::crossbeam_channel::{unbounded, Sender};

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        let (s, r) = unbounded();

        let pool = threadpool::ThreadPool::new(num_threads);

        for _ in 0..num_threads {
            let receiver = r.clone();

            pool.execute(move || loop {
                let th_message: ThreadMessage;

                match receiver.recv() {
                    Ok(val) => th_message = val,
                    Err(_e) => break,
                }
                if th_message.is_stateless() {
                    let info = th_message.get_stateless().unwrap();
                    (info.fun)(info.message);
                } else {
                    let info = th_message.get_stateful().unwrap();
                    let mut f = info.fun;
                    f(info.message);
                    info.sender.send((info.label, f)).unwrap();
                }
            });
        }

        ThreadPool { input_channel: s }
    }

    pub fn get_sender(&self) -> Sender<ThreadMessage> {
        self.input_channel.clone()
    }
}
