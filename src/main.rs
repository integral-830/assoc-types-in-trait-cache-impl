use core::{convert::Into, fmt::Debug, option::Option, result::Result::Ok};
use std::collections::HashMap;

trait Repository {
    type Item;
    type Id;
    type Error;

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error>;
    fn set(&mut self, id: Self::Id, item: Self::Item) -> Result<Self::Id, Self::Error>;
    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug)]
struct Employee {
    name: String,
    age: u64,
    email: String,
}

struct Memorycache {
    store: HashMap<u64, Employee>,
}

impl Memorycache {
    fn new() -> Self {
        Memorycache {
            store: HashMap::new(),
        }
    }
}

impl Repository for Memorycache {
    type Id = u64;
    type Item = Employee;
    type Error = std::convert::Infallible;

    fn set(&mut self, id: Self::Id, item: Self::Item) -> Result<Self::Id, Self::Error> {
        let id = id;
        self.store.insert(id, item);
        Ok(id.clone())
    }

    fn get(&self, id: &Self::Id) -> Result<Option<&Self::Item>, Self::Error> {
        Ok(self.store.get(id))
    }

    fn delete(&mut self, id: &Self::Id) -> Result<bool, Self::Error> {
        Ok(self.store.remove(id).is_some())
    }
}

fn create_and_fetch<R: Repository<Id = u64, Item = Employee>>(
    repo: &mut R,
    id: u64,
    value: Employee,
) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
    R::Error: std::fmt::Debug,
{
    let inserted_id = repo.set(id, value)?;
    print!("Inserted id: {inserted_id:?}\n");
    let recieved_value = repo.get(&inserted_id)?;
    let emp = recieved_value.unwrap();
    print!("recieved: {emp:?}");
    Ok(())
}

fn main() {
    let mut repo = Memorycache::new();
    let id = 1;
    create_and_fetch(
        &mut repo,
        id,
        Employee {
            name: "Ayush".into(),
            age: 21,
            email: "andy.ayushverma@gmail.com".into(),
        },
    )
    .unwrap();
}
