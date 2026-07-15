use std::path::Path;
use std::process::Command;
use std::fs;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Structure représentant un commit Git
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub files_changed: Vec<String>,
    pub insertions: u32,
    pub deletions: u32,
}

/// Structure pour l'historique complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHistory {
    pub commits: Vec<GitCommit>,
    pub total_commits: u32,
    pub total_files_changed: u32,
    pub total_insertions: u32,
    pub total_deletions: u32,
}

/// Vérifie si un dossier est un dépôt Git
pub fn is_git_repository(project_path: &Path) -> bool {
    let git_dir = project_path.join(".git");
    git_dir.exists() && git_dir.is_dir()
}

/// Récupère l'historique des commits Git
pub fn get_git_history(project_path: &Path, limit: u32) -> Result<GitHistory, String> {
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    let mut commits = Vec::new();
    let mut total_insertions = 0u32;
    let mut total_deletions = 0u32;
    let mut total_files_changed = 0u32;

    // Format personnalisé pour git log
    // %H = hash complet, %an = nom auteur, %ai = date ISO, %s = message
    let format = "%H%n%an%n%ai%n%s%n---END---";
    
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

    if !output.status.success() {
        return Err("Échec de la récupération de l'historique Git".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let entries: Vec<&str> = stdout.split("---END---").collect();

    for entry in entries {
        let lines: Vec<&str> = entry.trim().lines().collect();
        if lines.len() < 4 {
            continue;
        }

        let hash = lines[0].to_string();
        let author = lines[1].to_string();
        let date = lines[2].to_string();
        let message = lines[3].to_string();

        let mut files_changed = Vec::new();
        let mut insertions = 0u32;
        let mut deletions = 0u32;

        // Parser les statistiques de fichiers (lignes après le message)
        for line in lines.iter().skip(4) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                // Format: insertions deletions filename
                if let (Ok(ins), Ok(del)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    insertions += ins;
                    deletions += del;
                    files_changed.push(parts[2].to_string());
                }
            }
        }

        total_insertions += insertions;
        total_deletions += deletions;
        total_files_changed += files_changed.len() as u32;

        commits.push(GitCommit {
            hash: hash[..7].to_string(), // Court hash (7 premiers caractères)
            author,
            date,
            message,
            files_changed,
            insertions,
            deletions,
        });
    }

    Ok(GitHistory {
        total_commits: commits.len() as u32,
        total_files_changed,
        total_insertions,
        total_deletions,
        commits,
    })
}

/// Récupère les fichiers modifiés récemment (non commités)
pub fn get_uncommitted_changes(project_path: &Path) -> Result<Vec<String>, String> {
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    let output = Command::new("git")
        .current_dir(project_path)
        .args(["status", "--porcelain"])
        .output()
        .map_err(|e| format!("Erreur d'exécution de git: {}", e))?;

    if !output.status.success() {
        return Err("Échec de la récupération des changements".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let files: Vec<String> = stdout
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Format: "XY filename" où X et Y sont des codes de statut
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() >= 2 {
                parts[1].trim().to_string()
            } else {
                line.to_string()
            }
        })
        .collect();

    Ok(files)
}

/// Récupère les statistiques Git globales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStats {
    pub total_commits: u32,
    pub total_contributors: u32,
    pub first_commit_date: String,
    pub last_commit_date: String,
    pub total_branches: u32,
}

pub fn get_git_stats(project_path: &Path) -> Result<GitStats, String> {
    if !is_git_repository(project_path) {
        return Err("Le projet n'est pas un dépôt Git".to_string());
    }

    // Nombre total de commits
    let total_commits_output = Command::new("git")
        .current_dir(project_path)
        .args(["rev-list", "--all", "--count"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    let total_commits = String::from_utf8_lossy(&total_commits_output.stdout)
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    // Nombre de contributeurs uniques
    let contributors_output = Command::new("git")
        .current_dir(project_path)
        .args(["shortlog", "-sn", "--all"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    let total_contributors = String::from_utf8_lossy(&contributors_output.stdout)
        .lines()
        .count() as u32;

    // Date du premier commit
    let first_commit_output = Command::new("git")
        .current_dir(project_path)
        .args(["log", "--reverse", "--format=%ai", "--max-count=1"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    let first_commit_date = String::from_utf8_lossy(&first_commit_output.stdout)
        .trim()
        .to_string();

    // Date du dernier commit
    let last_commit_output = Command::new("git")
        .current_dir(project_path)
        .args(["log", "-1", "--format=%ai"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
    let last_commit_date = String::from_utf8_lossy(&last_commit_output.stdout)
        .trim()
        .to_string();

    // Nombre de branches
    let branches_output = Command::new("git")
        .current_dir(project_path)
        .args(["branch", "-a"])
        .output()
        .map_err(|e| format!("Erreur git: {}", e))?;
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

/// Structure pour un événement d'historique local
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHistoryEvent {
    pub timestamp: String,
    pub event_type: String, // "scan", "modification", "analysis"
    pub description: String,
    pub files_affected: Vec<String>,
}

/// Structure pour l'historique local complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalHistory {
    pub events: Vec<LocalHistoryEvent>,
    pub total_events: u32,
    pub last_scan_date: Option<String>,
    pub total_scans: u32,
}

/// Récupère l'historique local des modifications du projet
pub fn get_local_history(project_path: &Path) -> Result<LocalHistory, String> {
    let history_file = project_path.join(".scancode_history.json");
    
    if !history_file.exists() {
        return Ok(LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        });
    }

    let content = fs::read_to_string(&history_file)
        .map_err(|e| format!("Erreur lecture historique: {}", e))?;
    
    let history: LocalHistory = serde_json::from_str(&content)
        .map_err(|e| format!("Erreur parsing historique: {}", e))?;

    Ok(history)
}

/// Ajoute un événement à l'historique local
pub fn add_local_event(
    project_path: &Path, // Chemin du projet
    event_type: &str, // Type d'événement: "scan", "modification", "analysis"
    description: &str, // Description de l'événement
    files_affected: Vec<String>, // Liste des fichiers affectés
) -> Result<(), String> {
    let history_file = project_path.join(".scancode_history.json");
    
    let mut history = if history_file.exists() {
        let content = fs::read_to_string(&history_file)
            .map_err(|e| format!("Erreur lecture: {}", e))?;
        serde_json::from_str(&content).unwrap_or(LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        })
    } else {
        LocalHistory {
            events: Vec::new(),
            total_events: 0,
            last_scan_date: None,
            total_scans: 0,
        }
    };

    let timestamp = Utc::now().to_rfc3339();
    
    let event = LocalHistoryEvent {
        timestamp: timestamp.clone(),
        event_type: event_type.to_string(),
        description: description.to_string(),
        files_affected,
    };

    history.events.insert(0, event); // Ajouter en début pour avoir le plus récent en premier
    history.total_events += 1;

    if event_type == "scan" {
        history.last_scan_date = Some(timestamp);
        history.total_scans += 1;
    }

    // Limiter à 100 événements
    if history.events.len() > 100 {
        history.events.truncate(100);
    }

    let json = serde_json::to_string_pretty(&history)
        .map_err(|e| format!("Erreur serialization: {}", e))?;
    
    fs::write(&history_file, json)
        .map_err(|e| format!("Erreur écriture: {}", e))?;

    Ok(())
}

/// Détecte les modifications de fichiers depuis le dernier scan
pub fn detect_local_changes(project_path: &Path) -> Result<Vec<String>, String> {
    let history = get_local_history(project_path)?;
    
    if let Some(last_scan) = history.last_scan_date {
        let last_scan_time = DateTime::parse_from_rfc3339(&last_scan)
            .map_err(|e| format!("Erreur parsing date: {}", e))?;

        let mut changed_files = Vec::new();

        // Parcourir récursivement les fichiers du projet
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

        Ok(changed_files)
    } else {
        Ok(Vec::new())
    }
}