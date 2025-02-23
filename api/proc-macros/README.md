# proc-macros

Cette crate est une lib **proc-macros** qui fournit des macros personnalisées pour faciliter le développement de l'API avec EF-Backend. Elle fait partie intégrante du projet et est utilisée pour générer automatiquement des implémentations de certaines fonctionnalités.

## Description

Cette crate fournit des macros qui peuvent être utilisées pour générer du code automatiquement dans des contextes spécifiques, comme l'implémentation de traits, la validation de champs ou encore la génération de code répétitif.

### Fonctionnalités principales

- **Macro Updatable**: Propose une méthode pour mettre à jour simplement un modèle de données.

### Macro Updatable

Pour utiliser la macro `Updatable`, vous devez l'importer dans votre module et l'utiliser sur une structure définisant un modèle de données. Voici un exemple d'utilisation :

```rust
use api_proc_macros::updatable;

#[derive(Updatable]
struct User {
    id: i32,
    name: String,
    email: String,
}
```

Il est également nécessaire d'implémenter une structure externe nommé `Updatable'nom du modèle'` comprennant les champs à mettre à jour avec pour type des Option<T>.
Pour finir, il est nécessaire d'ajouter l'annotation `#[updatable]` sur les champs de la structure du modèle.

```rust
#[derive(Updatable)]
struct User {
    id: i32,
    #[updatable]
    name: String,
    #[updatable]
    email: String,
}

struct UpdatableUser {
    name: Option<String>,
    email: Option<String>,
}
```

## Utilisation

Cette crate est déjà utilisée au sein de la crate `db` cependant, elle peut également être utilisée dans d'autres contextes pour générer automatiquement du code.

Si vous souhaitez utiliser cette crate dans votre projet, ajoutez-la en tant que dépendance dans votre fichier `Cargo.toml`:

```toml
[dependencies]
api-proc-macros = { path = "../proc-macros" }
```
