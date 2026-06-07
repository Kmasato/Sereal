pub trait DataUpdateHandler: Send + Sync + 'static {
    fn on_received(&self, data: Vec<u8>);

    fn on_error(&self, message: String);
}
