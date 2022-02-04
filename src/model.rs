#[derive(Queryable)]
pub struct Collection{
  pub id: i32;
  pub title: String,
  pub published: bool
}