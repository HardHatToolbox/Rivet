use std::sync::{Mutex, MutexGuard};
use std::ops::DerefMut;

pub fn with_lock<T, F, R>(lock: &Mutex<T>, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
{
    let mut guard = lock.lock().expect("Failed to acquire lock");
    f(guard.deref_mut())
}