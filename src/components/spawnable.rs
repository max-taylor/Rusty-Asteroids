pub struct Spawnable<T> {
    pub entities: Vec<T>,
}

impl<T> Spawnable<T> {
    pub fn spawn(&mut self, entity: T) -> &mut Self {
        self.entities.push(entity);

        self
    }
}

impl<T> Default for Spawnable<T> {
    fn default() -> Self {
        Self { entities: vec![] }
    }
}
