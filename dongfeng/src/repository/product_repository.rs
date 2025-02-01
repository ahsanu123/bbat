use diesel::{RunQueryDsl, SqliteConnection};

use crate::{models::Dregs, schema::dregs};

pub struct ProductRepository;
impl ProductRepository {
    pub fn add(conn: &mut SqliteConnection, product: Dregs) {
        let result = diesel::insert_into(dregs::table)
            .values(&product)
            .execute(conn);
    }
    pub fn update(produt: Dregs) {}
    pub fn delete(id: i32) {}
    pub fn gets_all() {}
}

#[cfg(test)]
mod test {
    use chrono::NaiveDate;

    use crate::establish_connection;

    use super::*;

    #[test]
    fn add_dummy_data() {
        let dregs = Dregs {
            username: String::from("me"),
            description: String::from("a some description"),
            id: 0,
            user_id: 1,
            count: 1,
            paid: false,
            taken_time: NaiveDate::from_ymd(2025, 2, 3),
            production_time: NaiveDate::from_ymd(2025, 2, 3),
            amount: 1,
            price: 11000,
        };

        let conn = &mut establish_connection();
        ProductRepository::add(conn, dregs);
    }
}
