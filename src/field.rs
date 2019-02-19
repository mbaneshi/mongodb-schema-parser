use super::{Bson, FieldType};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Field {
  pub name: String,
  pub path: String,
  pub count: usize,
  pub bson_types: Vec<Option<String>>,
  pub probability: f32,
  pub has_duplicates: bool,
  pub types: Vec<FieldType>,
}

impl Field {
  pub fn new(name: String, path: &str) -> Self {
    Field {
      name: name,
      count: 1,
      path: path.to_string(),
      bson_types: Vec::new(),
      probability: 0.0,
      has_duplicates: false,
      types: Vec::new(),
    }
  }

  pub fn create_type(&mut self, value: &Bson) {
    let field_type =
      FieldType::new(&self.path, &value).add_to_type(&value, self.count);
    self.bson_types.push(field_type.bsonType.clone());
    self.types.push(field_type.to_owned())
  }

  pub fn does_field_type_exist(&mut self, field_type: &Option<String>) -> bool {
    self.bson_types.contains(&field_type)
  }

  pub fn get_path(name: String, path: &Option<String>) -> String {
    match &path {
      None => name,
      Some(path) => {
        let mut path = path.clone();
        path.push_str(".");
        path.push_str(&name);
        path
      }
    }
  }

  pub fn set_probability(&mut self, parent_count: usize) {
    self.probability = self.count as f32 / parent_count as f32
  }

  pub fn update_count(&mut self) {
    self.count += 1
  }

  pub fn set_duplicates(&mut self, duplicates: bool) {
    self.has_duplicates = duplicates
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::Bencher;

  #[test]
  fn it_creates_new() {
    let path = "Nori.cat";
    let count = 1;

    let field = Field::new("Nori".to_string(), &path);

    assert_eq!(field.name, "Nori".to_string());
    assert_eq!(field.path, path);
    assert_eq!(field.count, count);
  }

  #[bench]
  fn bench_it_creates_new(bench: &mut Bencher) {
    let path = "Nori.cat";

    bench.iter(|| Field::new("Nori".to_string(), &path));
  }

  #[test]
  fn it_gets_path_if_none() {
    let path = Field::get_path(String::from("address"), &None);
    assert_eq!(path, String::from("address"));
  }

  #[test]
  fn it_gets_path_if_some() {
    let path = Field::get_path(
      String::from("postal_code"),
      &Some(String::from("address")),
    );
    assert_eq!(path, String::from("address.postal_code"));
  }

  #[bench]
  fn bench_it_gets_path(bench: &mut Bencher) {
    bench.iter(|| {
      Field::get_path(
        String::from("postal_code"),
        &Some(String::from("address")),
      )
    });
  }

  #[test]
  fn it_sets_duplicates() {
    let mut field = Field::new("Rey".to_string(), "Rey.dog");
    field.set_duplicates(true);
    assert_eq!(field.has_duplicates, true)
  }

  #[bench]
  fn bench_it_sets_duplicates(bench: &mut Bencher) {
    let mut field = Field::new("Rey".to_string(), "Rey.dog");
    bench.iter(|| field.set_duplicates(true))
  }

  #[test]
  fn it_updates_count() {
    let mut field = Field::new("Chashu".to_string(), "Chashu.cat");
    field.update_count();
    assert_eq!(field.count, 2);
  }

  #[bench]
  fn bench_it_updates_count(bench: &mut Bencher) {
    let mut field = Field::new("Chashu".to_string(), "Chashu.cat");
    bench.iter(|| field.update_count());
  }

  #[allow(clippy::float_cmp)]
  #[test]
  fn it_sets_probability() {
    let mut field = Field::new("Nori".to_string(), "Nori.cat");
    field.set_probability(10);
    assert_eq!(field.probability, 0.1);
  }
}