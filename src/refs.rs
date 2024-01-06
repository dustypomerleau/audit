#![allow(dead_code)]

// shared pointer types

// Rc
// immutable, singlethread
fn rc() {
    use std::rc::Rc;

    let example_string = "example string".to_string();
    let rc_a: Rc<String> = Rc::new(example_string);
    let _rc_b: Rc<String> = Rc::clone(&rc_a);
}

// Arc
// immutable, multithread
fn arc() {
    use std::{sync::Arc, thread};

    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}

// Rc<RefCell>
// interior mutable, singlethread, runtime-tracked borrows
//
// For small, Copy types, like numbers, use Rc<Cell> (compile-time-checked borrows)
// but Cell swaps with default() for every get() on non-Copy types
//
// Rc<OnceCell> works without runtime checks, but you can only set it once.
fn rc_refcell() {
    use std::{
        cell::{RefCell, RefMut},
        collections::HashMap,
        rc::Rc,
    };

    let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
    // Create a new block to limit the scope of the dynamic borrow
    {
        let mut map: RefMut<'_, _> = shared_map.borrow_mut();
        map.insert("africa", 92388);
        map.insert("kyoto", 11837);
        map.insert("piccadilly", 11826);
        map.insert("marbles", 38);
    }

    // Note that if we had not let the previous borrow of the cache fall out
    // of scope then the subsequent borrow would cause a dynamic thread panic.
    // This is the major hazard of using `RefCell`.
    let total: i32 = shared_map.borrow().values().sum();
    println!("{total}");
}

// Arc<Mutex>
//
// Both Mutex and RwLock always lock.
// Mutex only allows 1 reader or writer at a time.
// RwLock allows 1 write lock or multiple read locks.
//
// Because of this, Mutex<T: Send> is Sync, RwLock<T: Send + Sync> is Sync.
// Requiring T: Sync allows RwLock to support concurrent reads.
//
// In other words, all Mutex locks give you a mutable reference to T,
// whereas RwLock allows you to choose between mutable and immutable references.
fn arc_mutex() {
    use std::{
        sync::{mpsc::channel, Arc, Mutex},
        thread,
    };

    const N: usize = 10;

    // Spawn a few threads to increment a shared variable (non-atomically), and
    // let the main thread know once all increments are done.
    //
    // Here we're using an Arc to share memory among threads, and the data inside
    // the Arc is protected with a mutex.
    let data = Arc::new(Mutex::new(0));

    let (tx, rx) = channel();
    for _ in 0..N {
        let (data, tx) = (Arc::clone(&data), tx.clone());
        thread::spawn(move || {
            // The shared state can only be accessed once the lock is held.
            // Our non-atomic increment is safe because we're the only thread
            // which can access the shared state when the lock is held.
            //
            // We unwrap() the return value to assert that we are not expecting
            // threads to ever fail while holding the lock.
            let mut data = data.lock().unwrap();
            *data += 1;
            if *data == N {
                tx.send(()).unwrap();
            }
            // the lock is unlocked here when `data` goes out of scope.
        });
    }

    rx.recv().unwrap();
}
