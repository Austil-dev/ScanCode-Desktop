# ScanCode Desktop
![Tauri](https://img.shields.io/badge/Tauri-black?style=plastic&logo=tauri&logoColor=yellow&labelColor=black&color=rgb(255%2C%20165%2C%200))
![Sveltekit](https://img.shields.io/badge/Sveltekit-black?style=plastic&logo=svelte&logoColor=orange&labelColor=black&color=orange)
![TypeScript](https://img.shields.io/badge/TypeScript-white?style=plastic&logo=typescript&logoColor=blue&labelColor=white&color=blue)
![Rust](https://img.shields.io/badge/Rust-white?style=plastic&logo=Rust&logoColor=black&labelColor=white&color=black)

## Présentation

**ScanCode Desktop** est une application de bureau permettant d'analyser rapidement la structure, les fichiers, les technologies et différentes caractéristiques techniques d'un projet logiciel.

L'objectif est de fournir une **vue synthétique d'un projet existant**, afin de faciliter sa compréhension avant une intervention, une maintenance ou une reprise de développement.

L'application analyse directement les fichiers présents sur le poste de l'utilisateur et présente les résultats dans différentes vues.

## Fonctionnalités

- Analyse de la structure du projet
- Identification des langages et types de fichiers
- Statistiques sur les fichiers et ressources
- Informations techniques détaillées
- Analyse de l'historique local
- Analyse de l'historique Git lorsqu'un dépôt est disponible
- Exploration de l'arborescence
- Calcul de différentes métriques de complexité
- Traitement local des données

## Technologies

- **Tauri**
- **Rust**
- **SvelteKit**
- **TypeScript**

## Architecture

ScanCode Desktop repose sur une architecture séparant l'interface utilisateur et les traitements réalisés côté natif.

### Frontend — SvelteKit + TypeScript

Le frontend est développé avec **SvelteKit** et **TypeScript**.

Il assure notamment :

- l'affichage des résultats d'analyse ;
- la navigation entre les différentes vues ;
- la présentation des statistiques ;
- l'exploration de l'arborescence ;
- l'affichage des informations techniques et historiques.

### Backend — Rust + Tauri

Le backend est développé en **Rust** et intégré à l'application via **Tauri**.

Il assure notamment :

- l'accès au système de fichiers ;
- l'analyse des fichiers et dossiers ;
- la récupération d'informations techniques ;
- le traitement des données Git ;
- le calcul des différentes métriques ;
- la communication entre le frontend et les fonctionnalités natives.

## Choix techniques

### Pourquoi Tauri ?

Tauri permet de développer une application de bureau multiplateforme en associant une interface web moderne à un backend natif.

Son utilisation permet notamment de conserver une application relativement légère tout en donnant accès aux fonctionnalités du système nécessaires à l'analyse des fichiers.

### Pourquoi Rust ?

Rust est utilisé pour les traitements nécessitant un accès natif au système et pour l'analyse locale des fichiers.

Il permet également de structurer la partie backend de l'application autour d'un langage fortement typé et adapté aux traitements systèmes.

### Pourquoi SvelteKit + TypeScript ?

SvelteKit permet de construire une interface réactive et organisée en plusieurs vues.

TypeScript apporte un typage statique permettant de mieux structurer les données échangées entre les différentes parties de l'application.

### Pourquoi un traitement local ?

Les données analysées correspondent directement aux fichiers présents sur le poste de l'utilisateur.

L'analyse est donc réalisée localement et le contenu des projets n'a pas besoin d'être transmis à un serveur distant pour effectuer les traitements.

## Fonctionnement

Lorsqu'un projet est ouvert dans ScanCode Desktop, l'application analyse son contenu et récupère différentes informations permettant d'en établir une vue d'ensemble.

Les résultats sont ensuite répartis dans quatre sections principales.

### Statistiques

Cette section présente différentes représentations graphiques permettant notamment d'observer :

- la répartition des types de fichiers ;
- les langages utilisés ;
- les ressources présentes ;
- l'évolution des modifications en fonction de la taille des fichiers et de la date ;
- les fichiers les plus modifiés.

### Informations

Cette section présente différentes caractéristiques techniques du projet, notamment :

- chemin du projet ;
- taille totale ;
- fichier le plus volumineux ;
- taille moyenne des fichiers ;
- profondeur maximale et moyenne de l'arborescence ;
- nombre total de fichiers ;
- nombre de fichiers de code ;
- nombre de ressources ;
- nombre de fichiers exécutables détectés ;
- proportion de fichiers de code ;
- complexité logicielle approximative ;
- IDE détecté ;
- langage dominant ;
- extension dominante ;
- informations Git disponibles.

### Données du projet

Cette section permet de consulter les données historiques disponibles.

Lorsqu'un dépôt Git est présent, les informations issues de Git peuvent être affichées.

L'application dispose également d'un historique local basé sur l'évolution de la taille des fichiers.

### Structure

Cette section permet d'explorer l'arborescence du projet.

Pour les fichiers accessibles, différentes informations peuvent être affichées :
- nom 
- extension 
- taille 
- contenu en mode lecture

## Analyse de complexité

ScanCode Desktop intègre une estimation approximative de la complexité logicielle d'un projet.

Plusieurs métriques sont prises en compte, notamment :

- **complexité cyclomatique** 
- **NPath complexity** 
- **LOC (Lines of Code)** 
- lignes exécutables 
- nombre d'opérateurs et d'opérandes 
- **volume de Halstead** 
- **index de maintenabilité**

Ces différentes métriques sont utilisées pour produire une estimation globale de la complexité du projet.

Cette analyse doit être considérée comme **indicative** et ne constitue pas une mesure exhaustive de la qualité ou de la complexité réelle d'un logiciel.

## Installation

### Prérequis

Le projet nécessite un environnement permettant de développer avec Tauri.

Les prérequis officiels peuvent être consultés dans la documentation de Tauri :

https://v2.tauri.app/fr/start/prerequisites/

### Installation des dépendances

Après avoir installé et configuré les prérequis Tauri, installer les dépendances du projet :

```bash
npm install
```

### Lancement du projet

Selon l'IDE et la configuration utilisées, le projet pourra être lancé différemment, mais si vous utiliser VS Code et que vous avez déjà installé et configuré Tauri, la commande est la suivante.

Pour lancer l'application dans un terminal VS Code, en mode développement :

```bash
npm run tauri dev
```

# Limitations

Le projet étant un prototype fonctionnel, certaines fonctionnalités restent volontairement limitées.

- La détection des IDE ne couvre pas tous les environnements existants.
- L'historique local repose sur les variations de taille des fichiers.
- De très petites modifications peuvent ne pas être suffisamment significatives pour apparaître dans l'historique local.
- L'historique Git n'est disponible que lorsqu'un dépôt Git est détecté.
- Certains caractères spéciaux et symboles ne sont actuellement pas pris en charge lors de la lecture de certains fichiers.
- Certains types de fichiers exécutables ne sont pas encore pris en compte.
- L'analyse de complexité constitue une estimation et peut varier selon le type de projet et les fichiers analysés.

# Structure du code

## 💻 Frontend

```bash
src/
├── routes/
│   ├── +page.svelte
│   ├── +layout.ts
│   └── stats/
│       ├── +page.svelte
│       ├── +layout.svelte
│       ├── VerticalToolbar.svelte
│       ├── donnees/
│       │   └── +page.svelte
│       ├── infos/
│       │   └── +page.svelte
│       └── structure/
│           └── +page.svelte
│
└── lib/
    └── components/
        └── PageTransition.svelte
```

### 📄 src/routes/+page.svelte
La page de présentation principale qui contient l'historique de navigation des projets, et des informations générales sur l'application ScanCode Desktop.

### 📄 src/routes/+layout.ts
Le fichier de configuration de la page principale, avec prerender = true pour charger dès le démarrage la page princiaple comme une page HTML statique, et ssr = false pour Tauri qui n'embarque pas de server 'Node.js'.

### 📄 src/lib/components/PageTransition.svelte
Une simple page de chargement qui a pour but de faire une transition rapide entre la page principale qui contient l'historique de navigation des projets, et la page 'Statistiques' qui contient les diagrammes.

### 📄 src/routes/stats/+page.svelte
La première page auquelle l'utilisateur accède lorsqu'il ouvre un projet. Elle contient un camembert qui résume le type de fichiers contenus dans le projet, un diagrammae d'évolution qui indique les modifications apportées au projet en fonction de la taille ajoutée (octets) et de la date, ainsi qu'un diagramme en barres, pour mettre en avant les fichiers les plus modifiés du projet.

### 📄 src/routes/stats/+layout.svelte
Layout commun aux différentes vues d'analyse.

Il regroupe les sections :
- Statistiques
- Informations
- Données
- Structure

### 📄 src/routes/stats/VerticalToolbar.svelte
Ce fichier contient la barre verticale située à gauche lors de la navigation dans un projet. Elle permet de passer facilement d'un onglet (encaspulé dans le layout) à un autre.

### 📄 src/routes/stats/donnees/+page.svelte
Ce fichier contient les données historiques locales et Git si elles sont trouvées. Dans le cas où il n'y a pas de répertoire Git, l'histoire Git ne sera pas trouvé. Dans le cas où aucune modification récente n'a été apporté au projet, l'historique local n'affichera pas de données. Il reste possible que l'historique local ne toruve pas de données à charger si les modifications apportées sont trop infimes par rapport à la taille du projet, puisque l'historique local vérifie si la taille des fichiers (en octets) a été modifiée.

### 📄 src/routes/stats/infos/+page.svelte
Le fichier qui contient le code permettant de visualiser des informations techniques et détaillées du projets. Il contient diverses informations : le chemin du projet, sa taille, le fichier le plus volumineux, la taille moyenne par fichier, la profondeur maximale du projet (nombre de dossiers à parcourir au maximum pour accéder à un fichier), la profondeur moyenne d'un fichier, le nombre total de fichiers, le nombre de fichiers de code, le nombre de fichiers 'ressources' (les images, les vidéos, la musique), le nombre de fichiers exécutables, (actuellement les .o ne sont pas pris en compte), le taux de fichiers de code (%), la complexité logicielle du projet approximative (plus le score se rapproche de 10, moins la complexité sera élevée), l'IDE utilisé, le langage de programmation dominant, l'extension dominante, et quelques informations Git.

Note : Certaines fonctionnalités restent simples et ne détectent pas tous les cas, comme pour les IDE (même s'il y a d'autre cas), où seulement quelques IDE ont été inclus, car inclure tous les IDE serait une charge inutile pour les besoins de ce projet qui reste un prototype.

### 📄 src/routes/stats/structure/+page.svelte
Le fichier structure contient l'arborescence du projet en clair avec le nom, l'extension et la taille de chaque dossier et fichier du projet. Le nom du projet est affiché en haut à gauche et la taille totale du projet en haut à droite. Ce fichier utilise des fonctions du backend depuis lib.rs pour permettre d'avoir des informations significatives sur un fichier : son nom, son extension, sa taille en octets, ainsi que le contenu du fichier en mode lecture.

Cette arborescence a été organisée pour visualiser facilement le nom, l'extension et surtout la taille de chaque dossier et fichier. Certains caractères spéciaux, et les symboles ne sont actuellement pas supportés pour la lecture.

## ⚙️ Backend

```bash
src-tauri/
└── src/
    ├── color.rs
    ├── complex.rs
    ├── history.rs
    ├── lib.rs
    └── main.rs
```

### 📄 src-tauri/src/color.rs
Color.rs contient le code couleur associé aux mots-clés des langages de programmation (ex : JavaScript) et à certains types de fichiers de l'explorateur de fichier (ex: Documents). pour une meilleure lisibilité du projet.

### 📄 src-tauri/src/complex.rs
Le fichier Rust qui contient les fonctions permettant de calculer et évaluer la complexité d'un projet. Pour mesure la complexité d'un projet, la complexité de chaque fichier est évalué en fonction de plusieurs facteurs déterminants : le nombre de structures de contrôle (complexité cyclomatique), le nombre de chemins d'exécutions avec les structures imbriquées (npath, ex: boucles for et while), le nombre de lignes de code et de lignes exécutables (métriques LOC, ignore les lignes vides), le nombre d'opérateurs et opérandes (volume d'Halstead), la difficulté à effectuer des opérations de maintenance (index de maintenabilité).

### 📄 src-tauri/src/history.rs
Ce fichier est dédié à la présentation des données concerçant l'historique Git d'un projet s'il y en a un.

### 📄 src-tauri/src/lib.rs
Le fichier lib.rs contient un grand nombre de fonctions spécifiques dédiées aux fichiers et dossiers, ainsi qu'à Git pour fournir à l'utilisateur des informations détaillées sur le contenu du projet, les détails et le contenu de chaque fichier. Il contient également des fonctionnalités permettant de trier les types et extensions de fichiers du projet pour les différencier.

Ce fichier contient des commandes Tauri peremettant de calculer et évaluer des informations techniques et détaillées communiquées entre le backend et le frontend.

### 📄 src-tauri/src/main.rs
Un simple appel de la fonction run() de lib.rs pour lancer le code Rust.

## Objectifs du projet

ScanCode Desktop a été développé comme un prototype fonctionnel d'outil d'analyse de projets logiciels.

Le projet permet notamment d'expérimenter :
- le développement d'applications de bureau avec Tauri 
- l'utilisation de Rust pour des traitements locaux 
- la communication entre une interface SvelteKit et un backend Rust 
- l'analyse du système de fichiers 
- l'exploitation de données Git 
- la production de métriques techniques à partir d'un projet existant


