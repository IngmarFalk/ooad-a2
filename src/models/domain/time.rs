/// ! This is experimental.
use crate::types::Model;

pub trait Listener<T> {
    fn notify(&mut self, new: T);
    fn pull(&self) -> T;
}

pub trait Listenable<V, T>
where
    T: Listener<V> + Model + Clone,
{
    fn add_listener(&mut self, listener: T);
    fn pop_listener(&mut self, listener: T) -> Option<T>;
    fn notify_listeners(&mut self);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Time<T>
where
    T: Listener<usize> + Model + Clone + PartialEq + Eq,
{
    listeners: Vec<T>,
    day: usize,
}

impl<T> Listenable<usize, T> for Time<T>
where
    T: Listener<usize> + Model + Clone + PartialEq + Eq,
{
    fn add_listener(&mut self, listener: T) {
        self.listeners.push(listener);
    }

    fn pop_listener(&mut self, listener: T) -> Option<T> {
        match self.index_of(&listener) {
            Some(i) => Some(self.listeners.remove(i)),
            None => None,
        }
    }

    fn notify_listeners(&mut self) {
        for listener in self.listeners.iter_mut() {
            listener.notify(self.day)
        }
    }
}

impl<T> Time<T>
where
    T: Listener<usize> + Model + Clone + PartialEq + Eq,
{
    pub fn new(listeners: Vec<T>, day: usize) -> Self {
        Self { listeners, day }
    }

    fn index_of(&self, val: &T) -> Option<usize> {
        self.listeners.iter().position(|v| v == val)
    }

    pub fn incr_day(&mut self) {
        self.day += 1;
    }
}
