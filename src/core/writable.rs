#[derive(Clone, PartialEq, Debug)]
pub struct Writable<T> {
    value: T,
    subscribers: Vec<fn(T)>,
}

impl<T: Clone> Writable<T> {
    /// Creates a new instance of the Writable struct
    ///
    /// # Arguments
    ///
    /// * `value`: The store's initial value
    ///
    /// returns: Writable<T>
    ///
    /// # Examples
    ///
    /// ```
    /// let store = Writable::new(10);
    /// ```
    pub fn new(value: T) -> Writable<T> {
        Writable {
            value,
            subscribers: Vec::new(),
        }
    }

    /// Subscribes to change notifications on the store instance
    ///
    /// # Arguments
    ///
    /// * `callback`: The function to call whenever the store's value changes
    ///
    /// returns: usize A unique identifier used for unsubscribing to change notifications via the `unsubscribe` method
    ///
    /// # Examples
    ///
    /// ```
    /// let store = Writable::new(10);
    /// let id = store.subscribe(|x| println!("{}", x));
    /// ```
    pub fn subscribe(&mut self, callback: fn(T)) -> usize {
        callback(self.value.clone());
        self.subscribers.push(callback);
        return self.subscribers.len() - 1;
    }
    /// Unsubscribes from change notifications
    ///
    /// # Arguments
    ///
    /// * `i`: A unique identifier, as returned by `subscribe`
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let store = Writable::new(10);
    /// let id = store.subscribe(|x| println!("{}", x));
    /// store.unsubscribe(id);
    /// ```
    pub fn unsubscribe(&mut self, i: usize) {
        self.subscribers.remove(i);
    }
    /// Sets the store's value and notifies all subscribers
    ///
    /// # Arguments
    ///
    /// * `value`: The new value
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let store = Writable::new(10);
    /// store.set(2);
    /// ```
    pub fn set(&mut self, value: T) {
        self.value = value.clone();

        for subscriber in self.subscribers.clone() {
            subscriber(value.clone());
        }
    }
}
