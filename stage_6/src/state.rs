// use crate::modelds::Course;
use sqlx::PgPool;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
    // pub courses: Mutex<Vec<Course>>,
}
