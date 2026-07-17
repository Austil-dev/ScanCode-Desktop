use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

// derive est un attribut qui permet de générer les implémentations pour FileComplexity pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Structure représentant un commit Git.
*/
pub struct GitCommit {
    pub hash: String,               // Hash du commit (7 caractères abrégés dans l'affichage)
    pub author: String,             // Auteur du commit
    pub date: String,               // Date du commit au format ISO
    pub message: String,            // Message de commit
    pub files_changed: Vec<String>, // Liste des fichiers modifiés
    pub insertions: u32,            // Nombre de lignes ajoutées
    pub deletions: u32,             // Nombre de lignes supprimées
}

// derive est un attribut qui permet de générer les implémentations pour GitHistory pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Structure représentant l'historique Git d'un projet.
*/
pub struct GitHistory {
    pub commits: Vec<GitCommit>,  // Liste des commits récupérés
    pub total_commits: u32,       // Nombre total de commits récupérés
    pub total_files_changed: u32, // Nombre total de fichiers modifiés
    pub total_insertions: u32,    // Nombre total de lignes ajoutées
    pub total_deletions: u32,     // Nombre total de lignes supprimées
}

/*
    Vérifie si le dossier passé en paramètre est un dépôt Git.
*/
pub fn is_git_repository(project_path: &Path) -> bool {
    // Vérifie si le dossier contient un sous-dossier .git pour déterminer s'il s'agit d'un dépôt Git.
    let git_dir = project_path.join(".git");
    // Vérifie si le chemin .git existe et est un répertoire
    git_dir.exists() && git_dir.is_dir()
}

/*
    Récupère l'historique des commits Git pour un projet.
*/
pub fn get_git_history(project_path: &Path, limit: u32) -> Result<GitHistory, String> {

    // Vérifie si le projet est un dépôt Git avant de récupérer l'historique.
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    let mut commits = Vec::new(); // Liste pour stocker les commits récupérés
    let mut total_insertions = 0u32;         // Nombre total de lignes ajoutées
    let mut total_deletions = 0u32;          // Nombre total de lignes supprimées
    let mut total_files_changed = 0u32;      // Nombre total de fichiers modifiés

    // Format personnalisé pour git log
    // %H = hash complet, %an = nom auteur, %ai = date ISO, %s = message
    let format = "%H%n%an%n%ai%n%s%n---END---";

    // Exécute la commande git log pour récupérer l'historique des commits avec le format spécifié et le nombre de commits limité.
    let output = Command::new("git")
        .current_dir(project_path)
        .args([
            "log",
            &format!("-{}", limit),
            &format!("--format={}", format),
            "--numstat",
        ])
        .output()
        .map_err(|e| format!("Erreur d'exécution de git: {}", e))?;

    // Vérifie si la commande git log a réussi
    if !output.status.success() {
        return Err("Échec de la récupération de l'historique Git".to_string());
    }

    // Récupère la sortie de la commande git log et la convertit en chaîne de caractères.
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Sépare les entrées de commit en utilisant le délimiteur "---END---"
    let entries: Vec<&str> = stdout.split("---END---").collect();

    // Parcourt chaque entrée de commit et extrait les informations pertinentes
    for entry in entries {

        // Nettoie les lignes vides et les espaces superflus
        let lines: Vec<&str> = entry.trim().lines().collect();

        // Vérifie que l'entrée contient au moins 4 lignes (hash, auteur, date, message)
        if lines.len() < 4 {
            continue;
        }

        let hash = lines[0].to_string();    // Hash complet du commit
        let author = lines[1].to_string();  // Nom de l'auteur du commit
        let date = lines[2].to_string();    // Date du commit au format ISO
        let message = lines[3].to_string(); // Message du commit

        let mut files_changed = Vec::new(); // Liste des fichiers modifiés pour ce commit
        let mut insertions = 0u32;                  // Nombre de lignes ajoutées pour ce commit
        let mut deletions = 0u32;                   // Nombre de lignes supprimées pour ce commit

        // Parser les statistiques de fichiers (lignes après le message)
        for line in lines.iter().skip(4) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // Format : insertions deletions filename
                if let (Ok(ins), Ok(del)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    insertions += ins;
                    deletions += del;
                    files_changed.push(parts[2].to_string());
                }
            }
        }

        total_insertions += insertions;                    // Mise à jour du total des insertions
        total_deletions += deletions;                      // Mise à jour du total des suppressions
        total_files_changed += files_changed.len() as u32; // Mise à jour du total des fichiers modifiés

        // Crée un objet GitCommit et l'ajoute à la liste des commits
        commits.push(GitCommit {
            hash: hash[..7].to_string(), // Abréviation du hash pour l'affichage
            author,
            date,
            message,
            files_changed,
            insertions,
            deletions,
        });
    }

    // Retourne l'historique Git complet avec les statistiques globales
    Ok(GitHistory {
        total_commits: commits.len() as u32,
        total_files_changed,
        total_insertions,
        total_deletions,
        commits,
    })
}

/*
    Récupère la liste des fichiers modifiés depuis le dernier commit.
*/
pub fn get_uncommitted_changes(project_path: &Path) -> Result<Vec<String>, String> {
    
    // Vérifie si le projet est un dépôt Git avant de récupérer les changements non commités.
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    // Exécute la commande git status --porcelain pour obtenir la liste des fichiers modifiés
    let output = Command::new("git")
        .current_dir(project_path)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("Erreur d'exécution de git: {}", e))?;

    // Vérifie si la commande git status a réussi
    if !output.status.success() {
        return Err("Échec de la récupération des changements".to_string());
    }

    // Récupère la sortie de la commande git status --porcelain et la convertit en chaîne de caractères.
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Filtre les lignes non vides et extrait les noms de fichiers modifiés.
    let files: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Format : "XY filename" où X et Y sont des codes de statut
            let parts: Vec<&str> = line.splitn(2, ' ').collect();

            // Si la ligne correspond au format attendu, on récupère le nom du fichier modifié.
            if parts.len() >= 2 {
                parts[1].trim().to_string() // Récupère le nom du fichier modifié
            }
            // Si la ligne ne correspond pas au format attendu, retourne la ligne entière. 
            else {
                line.to_string() // Cas inattendu, retourne la ligne entière
            }

        })
        .collect(); // Retourne la liste des fichiers modifiés depuis le dernier commit

    Ok(files) // Ok pour la gestion des erreurs
}

/// derive est un attribut qui permet de générer les implémentations pour GitStats pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Structure représentant les statistiques Git globales d'un projet.
*/
pub struct GitStats {
    pub total_commits: u32,        // Nombre total de commits dans le dépôt
    pub total_contributors: u32,   // Nombre de contributeurs uniques
    pub first_commit_date: String, // Date du premier commit
    pub last_commit_date: String,  // Date du dernier commit
    pub total_branches: u32,       // Nombre total de branches
}

/*
    Récupère les statistiques Git globales pour un projet.
*/
pub fn get_git_stats(project_path: &Path) -> Result<GitStats, String> {
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    // Nombre total de commits dans le dépôt.
    let total_commits_output = Command::new("git")
        .current_dir(project_path)
        .args(["rev-list", "--all", "--count"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    // Récupère le nombre total de commits et le convertit en entier.
    let total_commits = String::from_utf8_lossy(&total_commits_output.stdout)
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    // Nombre de contributeurs uniques dans le projet.
    let contributors_output = Command::new("git")
        .current_dir(project_path)
        .args(["shortlog", "-sn", "--all"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    // Compte le nombre de lignes dans la sortie de la commande git shortlog -sn --all pour obtenir le nombre total de contributeurs.
    let total_contributors = String::from_utf8_lossy(&contributors_output.stdout)
        .lines()
        .count() as u32;

    // Date du premier commit dans le dépôt.
    let first_commit_output = Command::new("git")
        .current_dir(project_path)
        .args(["log", "--reverse", "--format=%ai", "--max-count=1"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    // Récupère la date du premier commit et la convertit en chaîne de caractères.
    let first_commit_date = String::from_utf8_lossy(&first_commit_output.stdout)
        .trim()
        .to_string();

    // Date du dernier commit dans le dépôt.
    let last_commit_output = Command::new("git")
        .current_dir(project_path)
        .args(["log", "-1", "--format=%ai"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    // Récupère la date du dernier commit et la convertit en chaîne de caractères.
    let last_commit_date = String::from_utf8_lossy(&last_commit_output.stdout)
        .trim()
        .to_string();

    // Nombre total de branches disponibles.
    let branches_output = Command::new("git")
        .current_dir(project_path)
        .args(["branch", "-a"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    // Compte le nombre de lignes dans la sortie de la commande git branch -a pour obtenir le nombre total de branches.
    let total_branches = String::from_utf8_lossy(&branches_output.stdout)
        .lines()
        .count() as u32;

    Ok(GitStats {
        total_commits,
        total_contributors,
        first_commit_date,
        last_commit_date,
        total_branches,
    })
}

// derive est un attribut qui permet de générer les implémentations pour LocalHistoryEvent pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Structure représentant un événement d'historique local.
 */
pub struct LocalHistoryEvent {
    pub timestamp: String,           // Horodatage de l'événement au format RFC 3339
    pub event_type: String,          // Type d'événement : "scan", "modification", "analysis"
    pub description: String,         // Description de l'événement
    pub files_affected: Vec<String>, // Liste de fichiers concernés
}

// derive est un attribut qui permet de générer les implémentations pour LocalHistory pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/* 
    Structure représentant l'historique local d'un projet.
*/
pub struct LocalHistory {
    pub events: Vec<LocalHistoryEvent>, // Liste des événements
    pub total_events: u32,              // Nombre total d'événements
    pub last_scan_date: Option<String>, // Date du dernier scan
    pub total_scans: u32,               // Nombre total de scans
}

/* 
    Récupère l'historique local depuis le fichier `.scancode_history.json`.*/
pub fn get_local_history(project_path: &Path) -> Result<LocalHistory, String> {

    // Détermine le chemin du fichier d'historique local du projet.
    let history_file = project_path.join(".scancode_history.json");

    // Si le fichier d'historique n'existe pas, on retourne un historique vide.
    if !history_file.exists() {
        return Ok(LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        });
    }

    // Lecture du contenu du fichier d'historique local
    let content = fs::read_to_string(&history_file)
        .map_err(|e| format!("Erreur lecture historique: {}", e))?;

    // Désérialisation du contenu JSON en structure LocalHistory pour pouvoir l'utiliser dans le code
    let history: LocalHistory =
        serde_json::from_str(&content).map_err(|e| format!("Erreur parsing historique: {}", e))?;

    Ok(history) // Retourne l'historique local du projet
}

/* 
    Ajoute un événement local dans le fichier d'historique du projet.
 */
pub fn add_local_event(
    project_path: &Path,
    event_type: &str,
    description: &str,
    files_affected: Vec<String>,
) -> Result<(), String> {

    // Détermine le chemin du fichier d'historique local du projet.
    let history_file = project_path.join(".scancode_history.json");

    // Si le fichier existe, on lit l'historique existant, sinon on initialise un nouvel historique.
    let mut history = if history_file.exists() {
        let content =
            fs::read_to_string(&history_file).map_err(|e| format!("Erreur lecture: {}", e))?;
        serde_json::from_str(&content).unwrap_or(LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        })
    }
    // Si le fichier n'existe pas ou est vide, on initialise un nouvel historique local. 
    else {
        LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        }
    };

    // Récupère l'horodatage actuel au format RFC 3339.
    let timestamp = Utc::now().to_rfc3339();

    // Crée un nouvel événement local avec les informations fournies.
    let event = LocalHistoryEvent {
        timestamp: timestamp.clone(),
        event_type: event_type.to_string(),
        description: description.to_string(),
        files_affected,
    };

    // Ajoute l'événement en tête de la liste pour que le plus récent soit en premier.
    history.events.insert(0, event); // Ajouter en tête pour avoir le plus récent en premier.
    history.total_events += 1;

    // Met à jour la date du dernier scan et le compteur de scans si l'événement est un scan.
    if event_type == "scan" {
        history.last_scan_date = Some(timestamp);
        history.total_scans += 1;
    }

    // Limiter à 100 événements
    if history.events.len() > 100 {
        history.events.truncate(100);
    }

    // Sérialise l'historique en JSON et l'écrit dans le fichier d'historique.
    let json = serde_json::to_string_pretty(&history)
        .map_err(|e| format!("Erreur serialization: {}", e))?;

    // Écriture du fichier d'historique local
    fs::write(&history_file, json).map_err(|e| format!("Erreur écriture: {}", e))?;

    Ok(())
}

/* 
    Détecte les fichiers modifiés depuis le dernier scan local enregistré.
 */
pub fn detect_local_changes(project_path: &Path) -> Result<Vec<String>, String> {

    // Récupère l'historique local du projet pour déterminer la date du dernier scan.
    let history = get_local_history(project_path)?;

    // Si un dernier scan est enregistré, on compare les dates de modification des fichiers avec la date du dernier scan.
    if let Some(last_scan) = history.last_scan_date {
        let last_scan_time = DateTime::parse_from_rfc3339(&last_scan)
            .map_err(|e| format!("Erreur parsing date: {}", e))?;

        let mut changed_files = Vec::new();

        // Parcourir tous les fichiers du répertoire racine du projet.
        if let Ok(entries) = fs::read_dir(project_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Ok(modified) = metadata.modified() {
                            let modified_time: DateTime<Utc> = modified.into();
                            if modified_time.timestamp() > last_scan_time.timestamp() {
                                if let Some(path) = entry.path().file_name() {
                                    changed_files.push(path.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(changed_files) // Retourne la liste des fichiers modifiés depuis le dernier scan
    }
    // Si aucun scan précédent n'est trouvé, retourne une liste vide. 
    else {
        Ok(Vec::new()) // Aucun scan précédent, donc aucun fichier modifié à signaler
    }
}
