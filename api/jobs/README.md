# jobs

Cette crate fournit des outils pour implémenter des jobs dans votre application backend avec EF-Backend.

## Utilisation

Pour utiliser cette crate, si elle n'est pas présente dans votre crate bootstrap, vous devez ajouter la dépendance suivante à votre fichier Cargo.toml :

```toml
[dependencies]
api-jobs = { path = "../api/jobs" }
```

Dans votre fonction de bootstrap de l'application, vous allez devoir appeler la fonction `start_jobs` pour démarrer les jobs :

```rust
use api_jobs::start_jobs;

async fn main() {
    api_jobs::start_jobs(vec![HelloWorldJob::new(None)])
        .await
        .expect("Failed to start jobs");
}
```

La fonction `start_jobs` prend en paramètre une instance de la struct `Jobs` qui contient tous les jobs à démarrer. Vous pouvez créer une instance de `Jobs` en utilisant la méthode `new`.

Pour créer vos Jobs, vous devez implémenter la trait `Job` pour votre struct. Voici un exemple de job simple qui affiche un message dans la console tout les jours :

```rust
pub struct HelloWorldJob {
    schedule: String,
}

impl HelloWorldJob {
    pub fn new(
        schedule: Option<String>,
    ) -> Self {
        Self {
            schedule: schedule.unwrap_or_else(|| "* * 1/24 * * *".to_string()),
        }
    }
}

impl Job for HelloWorldJob {
    fn schedule(&self) -> String {
        self.schedule.clone()
    }

    fn run(
        &self,
        mut job_scheduler_lock: JobScheduler,
        uuid: Uuid,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            println!("Hello World!");

            // Query the next execution time for this job
            let _next_tick = job_scheduler_lock.next_tick_for_job(uuid).await;
        })
    }
}
```

### Cas plus complexe

Il est possible d'utiliser des services au sein du job, voici un exemple d'utilisation du service `UserService` :

```rust
pub struct UserServiceJob {
    schedule: String,
    user_service: UserService,
}

impl UserServiceJob {
    pub fn new(
        user_service: UserService,
        schedule: Option<String>,
    ) -> Self {
        Self {
            schedule: schedule.unwrap_or_else(|| "* * 1/24 * * *".to_string()),
            user_service,
        }
    }
}

impl Job for UserServiceJob {
    fn schedule(&self) -> String {
        self.schedule.clone()
    }

    fn run(
        &self,
        mut job_scheduler_lock: JobScheduler,
        uuid: Uuid,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let user_service = self.user_service.clone();

        Box::pin(async move {
            let users = user_service.get_all_users().await;

            for user in users {
                println!("User: {}", user.name);
            }

            // Query the next execution time for this job
            let _next_tick = job_scheduler_lock.next_tick_for_job(uuid).await;
        })
    }
}
```

> [!IMPORTANT]
> Il est important de cloner le service avant le Box::pin(async move {}).
