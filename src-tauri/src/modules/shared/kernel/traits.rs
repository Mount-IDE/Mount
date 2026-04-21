use crate::modules::shared::kernel::entities::LogLevel;

pub trait TError<T> {
    fn message(&self) -> T;
    fn level(&self) -> LogLevel;
}
