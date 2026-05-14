use super::data_classes::Container;

pub fn content<T>(object: &Container<T>) -> &str {
    object.content()
}

pub fn locked<T>(object: &Container<T>) -> bool {
    object.locked()
}

pub fn len<T>(object: &Container<T>) -> usize {
    object.values().len()
}
