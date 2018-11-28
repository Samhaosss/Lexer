#[derive(Debug)]
pub enum ErrType {
    IllegalCh,
}
pub struct ErrorHandler {
    err: ErrType,
}
impl ErrorHandler {
    pub fn handle(err_type: ErrType, error_msg: &str) {
        eprintln!("ERROR:{:?},{}", err_type, error_msg);
    }
}
