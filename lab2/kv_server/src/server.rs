use crate::storage::Storage;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct Server {
    pub storage: Arc<Storage>,
}
