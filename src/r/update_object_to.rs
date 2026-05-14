pub trait UpdateObjectTo<T> {
    fn update_object_to(self, template: T) -> T;
}

impl<T> UpdateObjectTo<T> for T {
    fn update_object_to(self, _template: T) -> T {
        self
    }
}
