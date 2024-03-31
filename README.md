# EF-backend 

Ce projet est une application web construite avec le framework Actix-web en Rust. Il utilise l'ORM Diesel pour interagir avec une base de données PostgreSQL et implémente une authentification basée sur des tokens JWT.

## Structure du projet 

Le projet est organisé en plusieurs modules principaux :

- [`auth`](src/auth/): Contient le code pour l'authentification et la gestion des tokens JWT.
- [`config`](src/config.rs): Gère la configuration de l'application.
- [`db`](src/db/): Contient le code pour la connexion à la base de données et les opérations CRUD.
- [`handlers`](src/handlers/): Contient les gestionnaires pour les différentes routes de l'API.
- [`models`](src/models/): Définit les structures de données utilisées dans l'application.
- [`types`](src/types/): Définit les types personnalisés utilisés dans l'application.

## Fonctionnalités principales

- Authentification basée sur des tokens JWT.
- Gestion des utilisateurs (création, récupération).
- Middleware pour la validation des tokens JWT.
- Gestion des erreurs personnalisée.

  if you want to start with hot reload, use this command in your terminal:  
`cargo watch -q -c -w src/ -x run`

## Améliorations futures

- **Pagination** : Si votre application doit gérer un grand nombre d'utilisateurs, vous pourriez envisager d'ajouter une pagination aux endpoints qui renvoient plusieurs utilisateurs.

- **Tests plus complets** : Bien que votre application ait des tests, vous pourriez envisager d'ajouter des tests plus complets, y compris des tests d'intégration et des tests de bout en bout.

- **Journalisation** : Pour faciliter le débogage, vous pourriez ajouter une journalisation plus détaillée de ce qui se passe lors de l'exécution de votre application.

- **Documentation de l'API** : Pour faciliter l'utilisation de votre API par d'autres développeurs (ou par vous-même à l'avenir), vous pourriez documenter vos endpoints d'API, par exemple en utilisant Swagger.

## Comment exécuter le projet

1. Assurez-vous d'avoir Rust et Diesel CLI installés sur votre machine.
2. Clonez ce dépôt.
3. Configurez votre base de données PostgreSQL et mettez à jour le fichier `.env` avec vos informations de connexion à la base de données.
4. Exécutez `diesel setup` pour créer la base de données.
5. Exécutez `cargo run` pour démarrer le serveur.

## Tests

Pour exécuter les tests, utilisez la commande `cargo test`.

## Licence

Ce projet est privé et n'est pas destiné à être distribué ou utilisé par d'autres personnes sans permission.
