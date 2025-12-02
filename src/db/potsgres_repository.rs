use crate::models::Person;
use crate::schema::people::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::SelectableHelper;
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct PostgresRepository {
    pub conn: DbConnection,
}

impl PostgresRepository {
    pub fn get_pool() -> DbPool {
        dotenv().ok();

        let database_url = env::var("PG_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .expect("DATABASE_URL must be set");
            
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    }

    /// GET /pessoas/:id
    pub fn get_people_by_id(&mut self, person_id: Uuid) ->  Result<Person, diesel::result::Error> {
       people.find(person_id).select(Person::as_select()).first(&mut self.conn)
    }

    /// POST /pessoas
    pub fn create_people(&mut self, new_person: Person) -> Result<Person, diesel::result::Error> {
        diesel::insert_into(people)
            .values(new_person)
            .returning(Person::as_select())
            .get_result(&mut self.conn)
    }

    /// GET /pessoas?t=[:t]
    pub fn search_people(&mut self, term: &str) -> Result<Vec<Person>, diesel::result::Error> {
        people
            .filter(search.ilike(format!("%{}%", term)))
            .limit(50)
            .select(Person::as_select())
            .load(&mut self.conn)
    }

    /// GET /contagem-pessoas
    pub fn count_people(&mut self) -> Result<i64, diesel::result::Error> {
        people.count().get_result(&mut self.conn)
    }
}
