use core::{convert::Into, fmt::Debug, option::Option, result::Result::Ok};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, sleep};

trait Repository {
    type Item;
    type Id;
    type Error;

    async fn get(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error>;
    async fn set(&self, id: Self::Id, item: Self::Item) -> Result<Self::Id, Self::Error>;
    async fn delete(&self, id: &Self::Id) -> Result<bool, Self::Error>;
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    age: u64,
    email: String,
}

struct Memorycache {
    store: Mutex<HashMap<u64, Employee>>,
}

impl Memorycache {
    fn new() -> Self {
        Memorycache {
            store: Mutex::new(HashMap::new()),
        }
    }
}

impl Repository for Memorycache {
    type Id = u64;
    type Item = Employee;
    type Error = std::convert::Infallible;

    async fn set(&self, id: Self::Id, item: Self::Item) -> Result<Self::Id, Self::Error> {
        sleep(Duration::from_millis(50)).await;
        let id = id;
        self.store.lock().await.insert(id, item);
        Ok(id.clone())
    }

    async fn get(&self, id: &Self::Id) -> Result<Option<Self::Item>, Self::Error> {
        sleep(Duration::from_millis(50)).await;
        let guard = self.store.lock().await;
        Ok(guard.get(id).cloned())
    }

    async fn delete(&self, id: &Self::Id) -> Result<bool, Self::Error> {
        sleep(Duration::from_millis(50)).await;
        Ok(self.store.lock().await.remove(id).is_some())
    }
}

async fn create_and_fetch<R: Repository<Id = u64, Item = Employee>>(
    repo: &R,
    id: u64,
    value: Employee,
) -> Result<(), R::Error>
where
    R::Item: std::fmt::Debug,
    R::Id: std::fmt::Debug,
    R::Error: std::fmt::Debug,
{
    let inserted_id = repo.set(id, value).await?;
    print!("Inserted id: {inserted_id:?}\n");
    let recieved_value = repo.get(&inserted_id).await?;
    let emp = recieved_value.unwrap();
    print!("recieved: {emp:?}\n");
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut repo = Arc::new(Memorycache::new());
    let repo_c1 = Arc::clone(&repo);
    let repo_c2 = Arc::clone(&repo);
    let id_1 = 1;
    let id_2 = 2;
    let t1 = tokio::spawn(async move {
        create_and_fetch(
            &*repo_c1,
            id_1,
            Employee {
                name: "Ayush".into(),
                age: 21,
                email: "andy.ayushverma@gmail.com".into(),
            },
        )
        .await
        .unwrap();
    });
    let t2 = tokio::spawn(async move {
        create_and_fetch(
            &*repo_c2,
            id_2,
            Employee {
                name: "Verma".into(),
                age: 21,
                email: "andy.ayushverma@gmail.com".into(),
            },
        )
        .await
        .unwrap();
    });

    t1.await.unwrap();
    t2.await.unwrap();
}
