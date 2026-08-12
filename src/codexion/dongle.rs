pub struct Dongle {}

struct DongleGuard<'a>(&'a mut Dongle);

impl Dongle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn acquire(&mut self) -> DongleGuard {
        DongleGuard(self)
    }

    fn release(&mut self) {
        todo!()
    }
}

impl<'a> Drop for DongleGuard<'a> {
    fn drop(&mut self) {
        self.0.release();
    }
}
