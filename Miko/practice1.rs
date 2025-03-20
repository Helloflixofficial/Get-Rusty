use sea_orm::{ActiveModelTrait, ActiveValue::Set, Database, EntityTrait, ModelTrait};

mod user;

use user::{ActiveModel as UserModel , Entity as User};

#[tokio::main]
async fn main() {
  let connection = Database::connect("postgres://mike:123456@localhost:5432/test").await.unwrap();

  let user1 = UserModel {
    name: Set("Jim".to_string()),
    ..Default::default()
  };

  user1.insert(&connection).await.unwrap();

  let user2 = UserModel {
    name: Set("Pam".to_string()),
    ..Default::default()
  };

  user2.insert(&connection).await.unwrap();

  let all_users = User::find().all(&connection).await.unwrap();
  println!("all users: {:?}", all_users);
  println!();

  let u1 = User::find_by_id(1).one(&connection).await.unwrap().unwrap();
  println!("u1: {:?}", u1);
  println!();

  u1.delete(&connection).await.unwrap();

  let mut u2: UserModel = User::find_by_id(2).one(&connection).await.unwrap().unwrap().into();

  u2.name = Set("Andy".to_string());
  u2.update(&connection).await.unwrap();

  let all_users = User::find().all(&connection).await.unwrap();
  println!("all users: {:?}", all_users);
  println!();
}
