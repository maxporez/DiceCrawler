# DiceCrawler — Conventions & Architecture

## Projet
- **Engine** : Bevy 0.18 (ECS)
- **Genre** : Dungeon Crawler / Dice Roguelike
- **Langue des commentaires** : français
- **Edition Rust** : 2024

## Architecture Bevy — Règles

### Organisation des fichiers
```
src/
  main.rs          — App::build() uniquement, add_plugins()
  plugins/         — un fichier par domaine (combat, map, ui…)
  components/      — types Component purs, sans logique
  systems/         — fonctions system, groupées par plugin
  resources/       — types Resource globaux
  events/          — types Event
  states/          — AppState enum + transitions
```

### Conventions ECS
- **1 responsabilité par system** : un system fait une seule chose.
- **Préférer les Events** aux queries croisées pour la communication entre systems.
- **Jamais de logique dans les Components** : ils sont des données pures.
- **SystemSet** pour ordonner explicitement : ne pas dépendre de l'ordre implicite.
- **Labels clairs** : nommer les SystemSet en verbes (`MovementSet`, `CombatSet`).

### States
- Utiliser `States` derive pour les phases du jeu (`MainMenu`, `Playing`, `GameOver`).
- Chaque plugin s'inscrit sur `OnEnter`/`OnExit`/`Update` avec `.run_if(in_state(...))`.

### Performance
- `Query` filtrée avec `With<>`/`Without<>` plutôt que des checks `.if let`.
- Éviter les `Commands` dans les boucles internes serrées — préférer un buffer.
- `Local<T>` pour l'état local d'un system (ex : timers internes).

## Rust — Règles

### Gestion d'erreurs
- Jamais de `.unwrap()` en dehors des tests et du `main()` de setup.
- Utiliser `thiserror` pour les erreurs de domaine.
- Les systems Bevy retournent `()` — logguer les erreurs avec `warn!` / `error!`.

### Style
- `snake_case` pour tout (fonctions, variables, fichiers).
- Structs/Enums en `PascalCase`.
- Pas de `pub` inutile : exposer au minimum nécessaire.
- Grouper les `use` : std → externe → interne.

### Pièges courants Bevy
- `Changed<T>` et `Added<T>` sont des filtres de query, pas des events — les utiliser pour les réactions locales seulement.
- `AssetServer::load` est asynchrone : ne pas supposer que l'asset est prêt au frame suivant.
- `Commands` sont exécutées en fin de stage — ne pas lire une entité spawn dans le même system.
- `ResMut` exclusive : éviter de tenir une `ResMut` et une `Res` du même type simultanément.

## Workflow
- Compiler avec `cargo check` avant `cargo run` pour valider rapidement.
- Profiling : activer `bevy/trace` uniquement en branche dédiée.
- Assets dans `assets/` (images, sons, maps) — jamais dans `src/`.
