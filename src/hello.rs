use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::{Duration, Instant},
};

struct Dongle {
    last_release: Mutex<Instant>,
    cv: Condvar,
    cooldown: Duration,
}

impl Dongle {
    fn acquire(&self) {
        let mut guard = self.last_release.lock().unwrap();
        loop {
            let elapsed = Instant::now().duration_since(*guard);
            if elapsed >= self.cooldown {
                break;
            }
            let wait = self.cooldown - elapsed;
            let (g, _) = self.cv.wait_timeout(guard, wait).unwrap();
            guard = g;
        } // acquired: guard held; proceed
    }

    fn release(&self) {
        let mut guard = self.last_release.lock().unwrap();
        *guard = Instant::now();
        self.cv.notify_all();
    }
}

fn main() {
    let d = Arc::new(Dongle {
        last_release: Mutex::new(Instant::now()),
        cv: Condvar::new(),
        cooldown: Duration::from_millis(500),
    });
    for id in 0..3 {
        let d2 = d.clone();
        thread::spawn(move || {
            d2.acquire();
            println!("coder {id} acquired");
            thread::sleep(Duration::from_millis(200));
            d2.release();
        });
    }
    thread::sleep(Duration::from_secs(2));
}
