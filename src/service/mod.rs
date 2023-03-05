pub mod cache_service;
pub mod redis_service;
pub mod sys_user_service;

#[macro_export]
macro_rules! pool {
    () => {
        &mut $crate::context::CONTEXT.rb().clone()
    };
}
