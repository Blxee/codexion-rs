pub struct Dongle {}

pub struct DongleGuard<'a>(&'a Dongle);

impl Dongle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn acquire(&self) -> DongleGuard {
        println!("dongle acquired");
        DongleGuard(self)
    }

    fn release(&self) {
        println!("dongle released");
    }
}

impl<'a> Drop for DongleGuard<'a> {
    fn drop(&mut self) {
        self.0.release();
    }
}
