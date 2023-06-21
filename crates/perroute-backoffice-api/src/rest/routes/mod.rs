use perroute_commons::rest::RestError;

//pub mod connections;
pub mod channels;
//pub mod connections;
pub mod health;
//pub mod plugins;

pub trait RestErrorHandler<E> {
    fn handle(error: E) -> RestError;
}
