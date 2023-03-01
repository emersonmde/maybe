pub trait KVStore<K, V> {
    fn new() -> Self;
    fn get(&self, key: K) -> Option<V>;
    fn put(&mut self, key: K, value: V);
}
