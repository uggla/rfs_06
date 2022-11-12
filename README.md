# Rust From Scratch

Rust From Scratch est une série de vidéo faites par [@Imilnb](https://twitter.com/iMilnb) sur la découverte du langage Rust.

La méthode convient plutôt bien à des personnes connaissant bien le C et cherche a expliquer ce que fait le compilateur Rust en regardant 
comment celui-ci gère la mémoire virtuelle.
L'objectif est aussi d'expliquer le plus possible les choses sans jamais assener des vérités.

## Episode 05 & 06 & 07.


Mettre les liens ici des que dispo sur youtube.

Ce repository contient des exemples de code pour bien comprendre la notion de borrow checker.

```
.
├── problem                --> le problème de ce code.
├── problem_rfs_07         --> Lors du stream07 notre maitre sith troublé par la vision d'Harley Quinn est un peu
│                              tombé dans un piège, ce répertoire contient l'explication de ce qui se passe au niveau ownership quand
│                              on utilise un type primitif.
├── problem_rfs_07_bis     --> Explication de l'usage d'un type primitif avec passage par référence.
├── solution1              --> solution du problème avec passage avec une fonction.
├── solution2              --> solution avec un clonage.
├── solution3              --> solution avec passage de référence.
└── solution3_bis          --> solution avec passage de réference et type &str, ce que l'on fait 
                               habituellement en Rust explication sommaire
                               de la fonctionnalité de Deref Coercion qui permet d'avoir un
                               &str quand on déréférence un String.
```


N'hésitez pas à créer des issues si vous avez des questions ou une PR s'il il y a des erreurs.
