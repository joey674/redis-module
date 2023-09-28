use parking_lot::*;


fn main() {
    let lock = RwLock::new(0);
    {
        let mut write = lock.write();
        *write += 1;
    }

    {
        let read = lock.read();
        println!("{}", *read);
    }

}