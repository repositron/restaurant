use diesel::result::Error;
use crate::models::{Session, NewSession};
use diesel::prelude::*;
use crate::domain::establish_connection;
use crate::schema::*;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};


pub struct SessionService {}

impl SessionService {
    pub async fn create_session(new_session: NewSession) -> Result<Session, Error> {
        use crate::schema::session::dsl::*;

        let insert = diesel::insert_into(session)
            .values(new_session)
            .get_result(&establish_connection())?;
        Ok(insert)
    }

    pub async fn end_session(session_id: Uuid) -> QueryResult<usize> {
        use crate::schema::session::dsl::*;
        let query = diesel::update(session.filter(session_id.eq(session_id)))
            .set(finish_timestamp.eq(Utc::now().naive_utc()))
            .execute(&establish_connection())?;
        Ok(query)
    }

    pub async fn get_session(session_id: i32) -> Result<Session, Error> {
        use crate::schema::session::dsl::*;
        let query = session
            .select(session::all_columns())
            .find(session_id)
            .get_result(&establish_connection())?;
        Ok(query)

    }

    pub async fn get_all_session() -> Result<Vec<Session>, Error> {
        use crate::schema::session::dsl::*;
        let query = session
            .select(session::all_columns())
            .load::<Session>(&establish_connection())?;
        Ok(query)
    }
}