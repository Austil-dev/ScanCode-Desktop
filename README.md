# ScanCode Desktop
![Tauri](https://img.shields.io/badge/Tauri-black?style=plastic&logo=tauri&logoColor=yellow&labelColor=black&color=rgb(255%2C%20165%2C%200))
![Sveltekit](https://img.shields.io/badge/Sveltekit-black?style=plastic&logo=svelte&logoColor=orange&labelColor=black&color=orange)
![TypeScript](https://img.shields.io/badge/TypeScript-white?style=plastic&logo=typescript&logoColor=blue&labelColor=white&color=blue)
![Rust](https://img.shields.io/badge/Rust-white?style=plastic&logo=Rust&logoColor=black&labelColor=white&color=black)

ScanCode Desktop est une application de bureau qui permet d'obtenir un tableau de bord rapide avec des données détaillées et techniques sur un projet de développeur, supportant un grand nombre de langages de programmation.
Ce logiciel utilise la stack Tauri (Sveltekit + Rust). 

Pour le frontend, TypeScript permet le typage et le contrôle du code, et svelte permet de naviguer dans un projet avec une interface moderne et réactive. Pour le backend, Rust permet d'accéder directement aux données techniques du projet en local-first. 

# Partie frontend
## src/routes/+page.svelte
La page de présentation principale qui contient l'historique de navigation des projets, et des informations générales sur le projet ScanCode Desktop.

# Partie backend
## src-tauri/src/color.rs
Color.rs contient le code couleur associé aux mots-clés des langages de programmation (ex : JavaScript) et à certains types de fichiers de l'explorateur de fichier (ex: Documents). pour une meilleure lisibilité du projet.

## src-tauri/src/complex.rs
Le fichier Rust qui contient les fonctions permettant de calculer et évaluer la complexité d'un projet. Pour mesure la complexité d'un projet, la complexité de chaque fichier est évalué en fonction de plusieurs facteurs déterminants : le nombre de structures de contrôle (complexité cyclomatique), le nombre de chemins d'exécutions avec les structures imbriquées (npath, ex: boucles for et while), le nombre de lignes de code et de lignes exécutables (métriques LOC, ignore les lignes vides), le nombre d'opérateurs et opérandes (volume d'Halstead), la difficulté à effectuer des opérations de maintenance (index de maintenabilité).

## src-tauri/src/history.rs
Ce fichier est dédié à la présentation des données concerçant l'historique Git d'un projet s'il y en a un.

## src-tauri/src/lib.rs
Le fichier lib.rs contient un grand nombre de fonctions spécifiques dédiées aux fichiers et dossiers, ainsi qu'à Git pour fournir à l'utilisateur des informations détaillées sur le contenu du projet, les détails et le contenu de chaque fichier. Il contient également des fonctionnalités permettant de trier les types et extensions de fichiers du projet pour les différencier.

Ce fichier contient des commandes Tauri peremettant de calculer et évaluer des informations techniques et détaillées communiquées entre le backend et le frontend.

## src-tauri/src/main.rs
Un simple appel de la fonction run() de lib.rs pour lancer le code Rust.
