
Un gestionnaire de presse-papiers en Rust qui sauvegarde et affiche l’historique des textes copiés sur votre pc.

## Fonctionnalités

- Sauvegarde automatiquement la dernière copie du presse-papiers
- Affiche l’historique complet des textes sauvegardés

## Utilisation

Le projet utilise [`clap`] pour gérer les commandes en ligne de commande.

### Commandes disponibles

- `save` : récupère le contenu actuel du presse-papiers et le sauvegarde
- `list` : affiche tout l’historique enregistré

### Exemples

Sauvegarder la copie courante :

---bash

cargo run -- save   (sauvegarde le fichier copié dans le presse papier)
cargo run -- list   (affichage de l'historiquew de copie)

