# AtlasX

AtlasX est un outil créatif-tech qui scanne un répertoire,
cartographie les extensions de fichiers, et génère une documentation
(JSON, Markdown, HTML) sur la structure réelle d'un projet.

## Fonctionnalités

- Scan récursif d'un répertoire
- Statistiques par extension (nombre de fichiers, taille totale, exemples)
- Export JSON, Markdown, HTML
- Base pour un moteur de suggestions (normalisation, regroupement, refactorisation)

## Installation

```bash
cd cli
cargo build --release
