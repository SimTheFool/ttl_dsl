### Lib interpreter

- Rajouter un port "formatter"
- faire en sorte que les commandes renvoie quelque chose de différent selon le type de formatter choisit (utiliser l'associated type)

### Lib-ts

- Faire un test sur un interpreter avec un mocked resolver et un json formatter
- Faire un test sur un interpreter avec un filesystem resolver et un json formatter
- Vérifier que la commande puissent s'éxecuter en asynchrone, sinon gérer l'asynchrone

### Améliorations:

- Gérer un map <Layer, Rule> dans le visitor, plutôt que de remonter l'AST
