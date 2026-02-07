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

fn coder(dongle: Arc<(Mutex<Instant>, Condvar)>, nb: i32) {
    let (mx, cv) = &*dongle;
    let mut guard = mx.lock().unwrap();
    let mut next_avail = *guard;

    while Instant::now() < next_avail {
        (guard, _) = cv.wait_timeout(guard, next_avail - Instant::now()).unwrap();
        next_avail = *guard;
    }

    *guard = Instant::now() + Duration::from_millis(3000);
    drop(guard);

    println!("coder {nb} is coding..");
    sleep(Duration::from_secs(1));
    println!("coder {nb} finished coding..");
    cv.notify_all();
}

fn main2() {
    println!("started..");
    let dongle = Arc::new((
        Mutex::new(Instant::now() - Duration::from_secs(3)),
        Condvar::new(),
    ));

    let coder_dongle = Arc::clone(&dongle);
    let coder1 = spawn(|| coder(coder_dongle, 1));

    let coder_dongle = Arc::clone(&dongle);
    let coder2 = spawn(|| coder(coder_dongle, 2));

    let (mx, cv) = &*dongle;
    let mut guard = mx.lock().unwrap();

    spawn(|| {
        let start = Instant::now();
        for _ in 0..20 {
            println!("time: {:10?}", start.elapsed().as_secs());
            sleep(Duration::from_millis(500));
        }
    });

    for _ in 0..2 {
        guard = cv.wait(guard).unwrap();
        if guard.elapsed() < Duration::from_millis(2000) {
            println!("dongle cooling down..");
        }
    }

    coder1.join().unwrap();
    coder2.join().unwrap();
    println!("finished..");
}
