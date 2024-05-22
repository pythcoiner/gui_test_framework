use crate::item::Item;

pub enum StoreError {
    DuplicateItem,
}

#[allow(unused)]
pub trait Store<I, K>
where
    I: Item<K>,
    K: PartialEq + Copy,
{
    // To be reimplemented
    fn bucket(&mut self, kind: K) -> &mut Vec<I>;

    // To not  be reimplemented
    fn push(&mut self, item: I) -> Result<(), StoreError> {
        let bucket = self.bucket(item.kind());
        if bucket.iter().any(|i| i.name() == item.name()) {
            // TODO: Should we error here? if so we can change bucket type to HashMap...
            Err(StoreError::DuplicateItem)
        } else {
            bucket.push(item);
            Ok(())
        }
    }
    fn find(&mut self, name: &str, kind: K) -> Option<&I> {
        let bucket = self.bucket(kind);
        let item = bucket.iter().find(|i| i.name() == name && i.kind() == kind);
        item
    }
}
