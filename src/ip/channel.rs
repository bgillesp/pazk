use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct Message<T, R> {
    pub data: T,
    pub producer: R,
    pub consumer: R,
}

pub struct Coordinator<T: Clone, R: Copy>
{
    log: Arc<Mutex< Vec< Message<T, R> > >>,
}

impl <T: Clone, R: Copy> Coordinator<T, R>
{
    pub fn new() -> Coordinator<T, R> {
        let vec: Vec< Message<T, R> > = Vec::new();
        Coordinator {
            log: Arc::new(Mutex::new(vec))
        }
    }

    pub fn create_channel(&self, part1: R, part2: R) -> (Channel<T, R>, Channel<T, R>) {
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let lbc1 = Channel {
            tx: tx1,
            rx: rx2,
            me: part1,
            you: part2,
            log: Arc::clone(&self.log),
        };
        let lbc2 = Channel {
            tx: tx2,
            rx: rx1,
            me: part2,
            you: part1,
            log: Arc::clone(&self.log),
        };
        (lbc1, lbc2)
    }

    pub fn get_log(&self) -> &Arc<Mutex< Vec< Message<T, R> > >> {
        &self.log
    }
}

// logged, bidirectional channel
pub struct Channel<T: Clone, R: Copy> {
    tx: mpsc::Sender<T>,
    rx: mpsc::Receiver<T>,
    me: R,
    you: R,
    log: Arc<Mutex< Vec< Message<T, R> > >>,
}

impl <T: Clone, R: Copy> Channel<T, R> {
    pub fn send(&self, data: T) {
        let mut log = self.log.lock().unwrap();
        let msg = Message {
            data: data.clone(),
            producer: self.me,
            consumer: self.you,
        };
        log.push(msg);

        // send data down channel, ignore error if send fails
        self.tx.send(data.clone()).ok();
    }

    pub fn receive(&self) -> T {
        self.rx.recv().unwrap()
    }
}
