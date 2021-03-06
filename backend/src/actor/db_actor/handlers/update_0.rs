// /* external crates */

// /* external uses */
// use actix::Handler;
// use diesel::prelude::*;
// // use uuid::Uuid;

// /* internal crates */

// /* internal uses */
// use super::super::DbActor;
// use crate::models::queryable_user::QueryableUser;
// use crate::schema::users;
// use crate::messages::update::update_with_username::UpdateWithUsername;

// impl Handler<UpdateWithUsername> for DbActor {
//     type Result = QueryResult<QueryableUser>;

//     fn handle(&mut self, msg: UpdateWithUsername, _: &mut Self::Context) -> Self::Result {
//         let conn = self.get_connection();

//         return diesel::update(users::dsl::users)
//             .filter(users::dsl::username.eq(msg.username))
//             .set((
//                 users::dsl::username.eq(msg.new_username),
//                 users::dsl::passwd.eq(msg.new_passwd),
//             ))
//             .get_result::<QueryableUser>(&conn);
//     }
// }
