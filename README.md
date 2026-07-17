# ScanCode Desktop
![Tauri](https://img.shields.io/badge/Tauri-black?style=plastic&logo=tauri&logoColor=yellow&labelColor=black&color=rgb(255%2C%20165%2C%200))
![Sveltekit](https://img.shields.io/badge/Sveltekit-black?style=plastic&logo=svelte&logoColor=orange&labelColor=black&color=orange)
![TypeScript](https://img.shields.io/badge/TypeScript-white?style=plastic&logo=typescript&logoColor=blue&labelColor=white&color=blue)
![Rust](https://img.shields.io/badge/Rust-white?style=plastic&logo=Rust&logoColor=black&labelColor=white&color=black)

ScanCode Desktop est une application de bureau qui permet d'obtenir un tableau de bord rapide avec des données détaillées et techniques sur un projet de développeur, supportant un grand nombre de langages de programmation.
Ce logiciel utilise la stack Tauri (Sveltekit + Rust). 

Pour le frontend, TypeScript permet le typage et le contrôle du code, et svelte permet de naviguer dans un projet avec une interface moderne et réactive. Pour le backend, Rust permet d'accéder directement aux données techniques du projet en local-first. 

# 💻 Frontend
## 📄 src/routes/+page.svelte
La page de présentation principale qui contient l'historique de navigation des projets, et des informations générales sur le projet ScanCode Desktop.

## 📄 src/routes/+layout.ts
Le fichier de configuration de la page principale, avec prerender = true pour charger dès le démarrage la page princiaple comme une page HTML statique, et ssr = false pour Tauri qui n'embarque pas de server 'Node.js'.

## 📄 src/lib/components/PageTransition.svelte
Une simple page de chargement qui a pour but de faire une transition rapide entre la page principale qui contient l'historique de navigation des projets, et la page 'Statistiques' qui contient les diagrammes.

## 📄 src/routes/stats/+page.svelte
La première page auquelle l'utilisateur accède lorsqu'il ouvre un projet. Elle contient un camembert qui résume le type de fichiers contenus dans le projet, un diagrammae d'évolution qui indique les modifications apportées au projet en fonction de la taille ajoutée (octets) et de la date, ainsi qu'un diagramme en barres, pour mettre en avant les fichiers les plus modifiés du projet.

## 📄 src/routes/stats/+layout.svelte
Le fichier layout contient l'encapsulation des onglets d'un projet, c'est-à-dire les onglets : 'Statistiques', 'Informations', 'Données', et 'Structure'.

## 📄 src/routes/stats/VerticalToolbar.svelte
Ce fichier contient la barre verticale située à gauche lors de la navigation dans un projet. Elle permet de passer facilement d'un onglet (encaspulé dans le layout) à un autre.

## 📄 src/routes/stats/donnees/+page.svelte
Ce fichier contient les données historiques locales et Git si elles sont trouvées. Dans le cas où il n'y a pas de répertoire Git, l'histoire Git ne sera pas trouvé. Dans le cas où aucune modification récente n'a été apporté au projet, l'historique local n'affichera pas de données. Il reste possible que l'historique local ne toruve pas de données à charger si les modifications apportées sont trop infimes par rapport à la taille du projet, puisque l'historique local vérifie si la taille des fichiers (en octets) a été modifiée.

## 📄 src/routes/stats/infos/+page.svelte
Le fichier qui contient le code permettant de visualiser des informations techniques et détaillées du projets. Il contient diverses informations : le chemin du projet, sa taille, le fichier le plus volumineux, la taille moyenne par fichier, la profondeur maximale du projet (nombre de dossiers à parcourir au maximum pour accéder à un fichier), la profondeur moyenne d'un fichier, le nombre total de fichiers, le nombre de fichiers de code, le nombre de fichiers 'ressources' (les images, les vidéos, la musique), le nombre de fichiers exécutables, (actuellement les .o ne sont pas pris en compte), le taux de fichiers de code (%), la complexité logicielle du projet approximative (plus le score se rapproche de 10, moins la complexité sera élevée), l'IDE utilisé, le langage de programmation dominant, l'extension dominante, et quelques informations Git.

Note : Certaines fonctionnalités restent simples et ne détectent pas tous les cas, comme pour les IDE (même s'il y a d'autre cas), où seulement quelques IDE ont été inclus, car inclure tous les IDE serait une charge inutile pour les besoins de ce projet qui reste un prototype.

## 📄 src/routes/stats/structure/+page.svelte
Le fichier structure contient l'arborescence du projet en clair avec le nom, l'extension et la taille de chaque dossier et fichier du projet. Le nom du projet est affiché en haut à gauche et la taille totale du projet en haut à droite. Ce fichier utilise des fonctions du backend depuis lib.rs pour permettre d'avoir des informations significatives sur un fichier : son nom, son extension, sa taille en octets, ainsi que le contenu du fichier en mode lecture.

Cette arborescence a été organisée pour visualiser facilement le nom, l'extension et surtout la taille de chaque dossier et fichier. Certains caractères spéciaux, et les symboles ne sont actuellement pas supportés pour la lecture.

# ⚙️ Backend

## 📄 src-tauri/src/color.rs
Color.rs contient le code couleur associé aux mots-clés des langages de programmation (ex : JavaScript) et à certains types de fichiers de l'explorateur de fichier (ex: Documents). pour une meilleure lisibilité du projet.

## 📄 src-tauri/src/complex.rs
Le fichier Rust qui contient les fonctions permettant de calculer et évaluer la complexité d'un projet. Pour mesure la complexité d'un projet, la complexité de chaque fichier est évalué en fonction de plusieurs facteurs déterminants : le nombre de structures de contrôle (complexité cyclomatique), le nombre de chemins d'exécutions avec les structures imbriquées (npath, ex: boucles for et while), le nombre de lignes de code et de lignes exécutables (métriques LOC, ignore les lignes vides), le nombre d'opérateurs et opérandes (volume d'Halstead), la difficulté à effectuer des opérations de maintenance (index de maintenabilité).

## 📄 src-tauri/src/history.rs
Ce fichier est dédié à la présentation des données concerçant l'historique Git d'un projet s'il y en a un.

## 📄 src-tauri/src/lib.rs
Le fichier lib.rs contient un grand nombre de fonctions spécifiques dédiées aux fichiers et dossiers, ainsi qu'à Git pour fournir à l'utilisateur des informations détaillées sur le contenu du projet, les détails et le contenu de chaque fichier. Il contient également des fonctionnalités permettant de trier les types et extensions de fichiers du projet pour les différencier.

Ce fichier contient des commandes Tauri peremettant de calculer et évaluer des informations techniques et détaillées communiquées entre le backend et le frontend.

## 📄 src-tauri/src/main.rs
Un simple appel de la fonction run() de lib.rs pour lancer le code Rust.

# ▶️ Lancement du projet
Selon l'IDE et la configuration utilisées, le projet pourra être lancé différemment, mais si vous utiliser VS Code et que vous avez déjà installé et configuré Tauri, la commande est la suivante : 
npm run tauri dev

Si vous avez besoin d'aide pour lancer le projet, voici le lien vers la documentation Tauri : https://v2.tauri.app/fr/start/prerequisites/
