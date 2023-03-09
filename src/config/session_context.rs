use tokio::task_local;

task_local! {
    pub static SESSION_CONTEXT: Session;
}

// #[derive(Copy, Clone)]
// pub struct Session {
//     pub id: Option<i32>,
//     pub name: &'static str,
// }
#[derive(Clone,Debug)]
pub struct Session {
    pub id: Option<i32>,
    pub name: String,
}

pub fn get_session() -> Option<Session> {
    let session_result = SESSION_CONTEXT.try_with(|session| (session.clone()));
    if session_result.is_err() {
        return None;
    }

    return Some(session_result.unwrap());
}