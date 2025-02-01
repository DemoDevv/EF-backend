# EF-backend 

Ce projet est une application web construite avec le framework Actix-web en Rust. Il utilise l'ORM Diesel pour interagir avec une base de données PostgreSQL et implémente une authentification basée sur des tokens JWT et OAuth2.0.

## Structure du projet 

Le projet est organisé en plusieurs modules principaux :

- [`caches`](api/caches/): Contient les caches personnalisés pour optimiser certaines manipulations.
- [`config`](api/config.rs): Gère la configuration de l'application.
- [`db`](api/db/): Contient le code pour la connexion à la base de données et les opérations CRUD.
- [`errors`](api/errors/): Contient la définition des erreurs du backend.
- [`extractors`](api/extractors/): Contient les extractors personnalisés.
- [`handlers`](api/handlers/): Contient les gestionnaires pour les différentes routes de l'API.
- [`jobs`](api/jobs/): Contient les jobs asynchrones.
- [`middlewares`](api/middlewares/): Définit les middlewares personnalisés.
- [`services`](api/services/): Définit les services utilisés dans les handlers.
- [`types`](api/types/): Définit les types personnalisés utilisés dans l'application.

## Fonctionnalités principales

- Authentification basée sur des tokens JWT et avec OAuth 2.0.
- Gestion des utilisateurs en base de données.
- Middleware pour la validation des tokens JWT.
- Gestion des erreurs personnalisée.

Si vous voulez commencer avec le rechargement à chaud, utilisez cette commande dans votre terminal:  
`cargo watch -q -c -w src/ -x run`

## Améliorations futures

- [ ] **Pagination** : Ajouter une pagination aux endpoints qui renvoient plusieurs utilisateurs pour mieux gérer un grand nombre d'utilisateurs.  
- [ ] **Tests plus complets** : Ajouter des tests d'intégration et des tests de bout en bout pour renforcer la couverture des tests.  
- [ ] **Journalisation** : Améliorer la journalisation pour faciliter le débogage et suivre plus précisément l'exécution de l'application.  
- [ ] **Documentation de l'API** : Documenter les endpoints d'API, par exemple en utilisant Swagger, pour faciliter l'utilisation par d'autres développeurs (ou soi-même plus tard).  

## Comment exécuter le projet

1. Assurez-vous d'avoir Rust et Diesel CLI installés sur votre machine.
2. Clonez ce dépôt.
3. Configurez votre base de données PostgreSQL et mettez à jour le fichier `.env` avec vos informations de connexion à la base de données.
4. Exécutez `diesel setup` pour créer la base de données.
5. Exécutez `cargo run` pour démarrer le serveur.

## Tests

Il est nécessaire d'avoir créé une base de données uniquement pour les tests et d'avoir configuré la variable d'environnement DATABASE_TEST_URL.  

Pour exécuter les tests, utilisez la commande `cargo test`.
