// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod colors;
mod complex;
mod history;

use colors::{get_color_palette, ColorPalette};
use history::{
    add_local_event, detect_local_changes, get_git_history, get_git_stats, get_local_history,
    get_uncommitted_changes, is_git_repository, GitHistory, GitStats, LocalHistory,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use encoding_rs::{UTF_8, WINDOWS_1252};
use std::io::Read;

#[derive(Serialize, Deserialize)]
// Structure pour représenter les modifications du projet par date
struct Modification {
    date: String,
    bytes: u64,
}

#[derive(Serialize, Deserialize, Clone)]
// Structure pour représenter le nombre de fichiers par type
struct FileTypeCount {
    extension: String, // Extension du fichier (ex: .rs, .js, .png, etc.)
    count: u32,        // Nombre de fichiers avec cette extension
}

#[derive(Serialize, Deserialize)]
// Structure pour représenter le contenu du projet par catégorie
struct ProjectContent {
    category: String, // Catégorie de fichiers (ex: Code, Ressources, Exécutables, etc.)
    files: Vec<FileTypeCount>, // Liste des types de fichiers et leur nombre dans cette catégorie
    total: u32,       // Nombre total de fichiers dans cette catégorie
}

#[derive(Serialize, Deserialize)]
// Structure pour représenter les fichiers les plus modifiés
struct ModifiedFile {
    file: String,       // Chemin relatif du fichier par rapport à la racine du projet
    modifications: u64, // Nombre de modifications (ou taille cumulée des modifications) pour ce fichier
}

#[derive(Serialize, Deserialize, Clone)]
// Structure pour représenter un nœud dans l'arborescence du projet
struct TreeNode {
    name: String,            // Nom du fichier ou du dossier
    path: String, // Chemin relatif du fichier ou du dossier par rapport à la racine du projet
    is_dir: bool, // Indique si c'est un dossier (true) ou un fichier (false)
    size: u64,    // Taille du fichier ou du dossier en octets
    children: Vec<TreeNode>, // Liste des enfants si c'est un dossier
}

#[derive(Serialize, Deserialize)]
// Structure pour représenter les informations globales du projet
struct ProjectInfo {
    path: String,              // Chemin absolu du projet
    size: u64,                 // Taille totale du projet en octets
    complexity: String,        // Complexité du projet (ex: "Acceptable • O(n)")
    code_files_count: u32,     // Nombre de fichiers de code
    resource_files_count: u32, // Nombre de fichiers ressources
    executable_count: u32,     // Nombre de fichiers exécutables
    total_files_count: u32,    // Nombre total de fichiers dans le projet
    code_files_percentage: f64, // Pourcentage de fichiers de code
    largest_file_path: String, // Chemin relatif du fichier le plus volumineux
    largest_file_size: u64,    // Taille du fichier le plus volumineux
    dominant_language: String, // Langage dominant du projet
    dominant_extension: String, // Extension de fichier dominante du projet
    average_depth: f64,        // Profondeur moyenne des fichiers dans l'arborescence
    max_depth: u32,            // Profondeur maximale de l'arborescence du projet
    detected_editor: String,   // Éditeur de code détecté pour le projet
}

/// Structure pour l'historique complet du projet
#[derive(Debug, Serialize, Deserialize)]
// Cette structure contient toutes les informations relatives à l'historique du projet
pub struct ProjectHistory {
    pub has_git: bool,                   // Indique si le projet est un dépôt Git
    pub git_history: Option<GitHistory>, // Historique Git du projet
    pub git_stats: Option<GitStats>,     // Statistiques Git du projet
    pub uncommitted_changes: Option<Vec<String>>, // Liste des fichiers modifiés mais non commités
    pub local_history: LocalHistory,     // Historique local du projet (non Git)
    pub recent_local_changes: Vec<String>, // Liste des fichiers récemment modifiés localement
}

/* Ouvre un projet et navigue vers la page des statistiques */
#[tauri::command]
async fn open_project(window: tauri::WebviewWindow, path: String) -> Result<String, String> {
    // Redimensionner la fenêtre
    window
        .set_size(tauri::PhysicalSize {
            width: 1600,
            height: 900,
        })
        .map_err(|e| format!("Erreur redimensionnement: {}", e))?;

    // Centrer la fenêtre
    window.center().ok(); // Ignorer les erreurs de centrage

    // Naviguer vers la page stats avec le chemin du projet
    let url = format!("stats?path={}", urlencoding::encode(&path));
    let js = format!("window.location.href = '{}';", url.replace('\'', "\\'"));
    window
        .eval(&js)
        .map_err(|e| format!("Erreur navigation: {}", e))?;

    Ok(format!("Projet ouvert: {}", path))
}

/* Retour vers le menu principal */
#[tauri::command]
async fn back_to_dashboard(window: tauri::WebviewWindow) -> Result<String, String> {
    // Redimensionner la fenêtre au retour
    window
        .set_size(tauri::PhysicalSize {
            width: 700,
            height: 700,
        })
        .map_err(|e| format!("Erreur redimensionnement: {}", e))?;

    // Centrer la fenêtre
    window.center().ok(); // Ignorer les erreurs de centrage

    // Naviguer vers le dashboard
    let js = "window.location.href = '/';";
    window
        .eval(js)
        .map_err(|e| format!("Erreur navigation: {}", e))?;

    Ok("Retour au dashboard".to_string())
}

/* Parcourir récursivement les fichiers du projet */
fn scan_directory(
    dir: &Path,
    language_stats: &mut HashMap<String, u64>,
    total_bytes: &mut u64,
    extensions_map: &HashMap<&str, &str>,
) -> Result<(), String> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            // Ignorer les dossiers communs à exclure
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name == ".git"
                    || name == "node_modules"
                    || name == "target"
                    || name == "dist"
                    || name == "build"
                    || name == "out"
                {
                    continue;
                }
            }

            // Si c'est un dossier, on continue la récursion
            if path.is_dir() {
                scan_directory(&path, language_stats, total_bytes, extensions_map)?;
            }
            // Si c'est un fichier, on vérifie son extension et on met à jour les statistiques
            else if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    if let Some(&language) = extensions_map.get(ext) {
                        if let Ok(metadata) = fs::metadata(&path) {
                            let size = metadata.len();
                            *language_stats.entry(language.to_string()).or_insert(0) += size;
                            *total_bytes += size;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// Extensions de fichiers et leurs langages correspondants
const EXTENSIONS: [(&str, &str); 32] = [
    ("rs", "Rust"),
    ("ts", "TypeScript"),
    ("tsx", "TypeScript"),
    ("js", "JavaScript"),
    ("jsx", "JavaScript"),
    ("svelte", "Svelte"),
    ("py", "Python"),
    ("java", "Java"),
    ("cpp", "C++"),
    ("cc", "C++"),
    ("cxx", "C++"),
    ("c", "C"),
    ("h", "C"),
    ("cs", "C#"),
    ("go", "Go"),
    ("php", "PHP"),
    ("rb", "Ruby"),
    ("swift", "Swift"),
    ("kt", "Kotlin"),
    ("css", "CSS"),
    ("scss", "CSS"),
    ("sass", "CSS"),
    ("html", "HTML"),
    ("htm", "HTML"),
    ("json", "JSON"),
    ("yaml", "YAML"),
    ("yml", "YAML"),
    ("xml", "XML"),
    ("md", "Markdown"),
    ("sql", "SQL"),
    ("sh", "Shell"),
    ("bash", "Shell"),
];

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction analyse les fichiers du projet pour déterminer la répartition des langages de programmation utilisés,
    en calculant le pourcentage de chaque langage basé sur la taille des fichiers.
*/
async fn analyze_project_languages(path: String) -> Result<HashMap<String, f64>, String> {
    let project_path = Path::new(&path); // Crée un objet Path à partir du chemin du projet fourni

    // Vérifier que le chemin du projet existe
    if !project_path.exists() {
        return Err("Le chemin du projet n'existe pas".to_string());
    }

    // Initialiser les statistiques
    let mut language_stats: HashMap<String, u64> = HashMap::new();
    // Initialiser le compteur total de bytes
    let mut total_bytes: u64 = 0;

    let extensions_map: HashMap<&str, &str> = EXTENSIONS.iter().cloned().collect();

    // Parcourir les fichiers du projet
    scan_directory(
        project_path,
        &mut language_stats,
        &mut total_bytes,
        &extensions_map,
    )
    .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Convertir en pourcentages
    let mut percentages: HashMap<String, f64> = HashMap::new();
    // Vérifier que le total de bytes est supérieur à zéro pour éviter la division par zéro
    if total_bytes > 0 {
        for (language, bytes) in language_stats {
            let percentage = (bytes as f64 / total_bytes as f64) * 100.0; // Calculer le pourcentage de chaque langage
            percentages.insert(language, (percentage * 100.0).round() / 100.0); // Arrondir à 2 décimales
        }
    }

    Ok(percentages)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction analyse le contenu du projet pour déterminer la répartition des fichiers par catégories et extensions,
    en comptant le nombre de fichiers pour chaque extension.
*/
async fn analyze_project_content(path: String) -> Result<Vec<ProjectContent>, String> {
    let project_path = Path::new(&path);

    if !project_path.exists() {
        return Err("Le chemin du projet n'existe pas".to_string());
    }

    let mut extension_counts: HashMap<String, u32> = HashMap::new();

    // Parcourir tous les fichiers et compter par extension
    fn scan_extensions(dir: &Path, counts: &mut HashMap<String, u32>) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Ignorer les dossiers à exclure
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == ".git"
                        || name == "node_modules"
                        || name == "target"
                        || name == "dist"
                        || name == "build"
                        || name == "out"
                    {
                        continue;
                    }
                }

                if path.is_dir() {
                    scan_extensions(&path, counts)?;
                } else if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext_lower = ext.to_lowercase();
                        *counts.entry(ext_lower).or_insert(0) += 1;
                    }
                }
            }
        }
        Ok(())
    }

    scan_extensions(project_path, &mut extension_counts)
        .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Catégoriser les extensions
    let mut categories: HashMap<String, Vec<FileTypeCount>> = HashMap::new();

    for (ext, count) in extension_counts {
        let category = match ext.as_str() {
            // Code C/C++
            "c" | "h" | "cpp" | "hpp" | "cc" | "cxx" | "hxx" => "C/C++",
            // Code autres langages
            "rs" => "Rust",
            "py" => "Python",
            "java" => "Java",
            "js" | "jsx" | "mjs" => "JavaScript",
            "ts" | "tsx" => "TypeScript",
            "cs" => "C#",
            "go" => "Go",
            "php" => "PHP",
            "rb" => "Ruby",
            "swift" => "Swift",
            "kt" | "kts" => "Kotlin",
            "svelte" => "Svelte",
            // Web
            "html" | "htm" => "HTML",
            "css" | "scss" | "sass" | "less" => "CSS",
            // Configuration
            "json" | "jsonc" => "Config",
            "yaml" | "yml" => "Config",
            "toml" => "Config",
            "xml" => "Config",
            "ini" | "cfg" | "conf" => "Config",
            // Exécutables et objets
            "exe" | "dll" | "so" | "dylib" => "Exec",
            "o" | "obj" | "a" | "lib" => "Exec",
            // Images
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" | "ico" | "webp" => "Images",
            // Audio
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => "Audio",
            // Vidéo
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" => "Video",
            // Documentation
            "md" | "txt" | "pdf" | "doc" | "docx" => "Docs",
            // Scripts
            "sh" | "bash" | "bat" | "cmd" | "ps1" => "Scripts",
            // Autres
            _ => "Autres",
        };

        categories
            .entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(FileTypeCount {
                extension: ext,
                count,
            });
    }

    // Convertir en résultat et trier
    let mut result: Vec<ProjectContent> = categories
        .into_iter()
        .map(|(category, mut files)| {
            files.sort_by(|a, b| b.count.cmp(&a.count));
            let total = files.iter().map(|f| f.count).sum();
            ProjectContent {
                category,
                files,
                total,
            }
        })
        .collect();

    // Trier par total décroissant
    result.sort_by(|a, b| b.total.cmp(&a.total));

    Ok(result)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction analyse les modifications du projet en fonction de la date de modification.
    Async est utilisé pour permettre l'exécution asynchrone, ce qui est utile pour les opérations de fichiers qui peuvent prendre du temps.
*/
async fn analyze_project_modifications(path: String) -> Result<Vec<Modification>, String> {
    let project_path = Path::new(&path);

    if !project_path.exists() {
        return Err("Le chemin du projet n'existe pas".to_string());
    }

    let mut modifications_by_month: HashMap<String, u64> = HashMap::new();
    let mut total_size: u64 = 0;

    // Parcourir tous les fichiers et agréger par date de modification
    fn scan_modifications(
        dir: &Path,
        modifications: &mut HashMap<String, u64>,
        total_size: &mut u64,
    ) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Ignorer les dossiers à exclure
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.starts_with('.')
                        || name == "node_modules"
                        || name == "target"
                        || name == "dist"
                        || name == "build"
                        || name == "out"
                        || name == ".git"
                    {
                        continue;
                    }
                }

                if path.is_dir() {
                    scan_modifications(&path, modifications, total_size)?;
                } else if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let size = metadata.len();
                        *total_size += size;

                        // Pour simplifier, on utilise la date de modification du système
                        // Dans une vraie implémentation, vous pourriez utiliser git pour obtenir l'historique
                        if let Ok(modified) = metadata.modified() {
                            use std::time::SystemTime;
                            if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                                let timestamp = duration.as_secs();
                                // Convertir en format YYYY-MM
                                let date = chrono::DateTime::from_timestamp(timestamp as i64, 0)
                                    .map(|dt| dt.format("%Y-%m").to_string())
                                    .unwrap_or_else(|| "2024-01".to_string());

                                *modifications.entry(date).or_insert(0) += size;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    scan_modifications(project_path, &mut modifications_by_month, &mut total_size)
        .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Convertir en liste triée par date et calculer les valeurs cumulatives
    let mut sorted_dates: Vec<String> = modifications_by_month.keys().cloned().collect();
    sorted_dates.sort();

    let mut cumulative_bytes: u64 = 0;
    let mut result: Vec<Modification> = Vec::new();

    for date in sorted_dates {
        if let Some(&bytes) = modifications_by_month.get(&date) {
            cumulative_bytes += bytes;
            result.push(Modification {
                date,
                bytes: cumulative_bytes,
            });
        }
    }

    // Si aucune modification n'est trouvée, retourner au moins une entrée
    if result.is_empty() {
        result.push(Modification {
            date: chrono::Local::now().format("%Y-%m").to_string(),
            bytes: total_size,
        });
    }

    Ok(result)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
   Cette fonction analyse les fichiers les plus modifiés en fonction de leur date de modification.
*/
fn analyze_modified_files(path: String) -> Result<Vec<ModifiedFile>, String> {
    let project_path = Path::new(&path);
    let mut file_modifications: HashMap<String, u64> = HashMap::new();

    fn scan_files(
        dir: &Path,
        base_path: &Path,
        file_mods: &mut HashMap<String, u64>,
    ) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Ignorer les dossiers communs à exclure
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == ".git"
                        || name == "node_modules"
                        || name == "target"
                        || name == "dist"
                        || name == "build"
                        || name == "out"
                    {
                        continue;
                    }
                }

                if path.is_dir() {
                    scan_files(&path, base_path, file_mods)?;
                } else if path.is_file() {
                    if let Ok(metadata) = fs::metadata(&path) {
                        let size = metadata.len();
                        if let Ok(relative_path) = path.strip_prefix(base_path) {
                            let path_str = relative_path.to_string_lossy().to_string();
                            *file_mods.entry(path_str).or_insert(0) += size;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    scan_files(project_path, project_path, &mut file_modifications)
        .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Convertir en vecteur et trier par nombre de modifications (taille)
    let mut result: Vec<ModifiedFile> = file_modifications
        .into_iter()
        .map(|(file, modifications)| ModifiedFile {
            file,
            modifications,
        })
        .collect();

    // Trier par modifications décroissant
    result.sort_by(|a, b| b.modifications.cmp(&a.modifications));

    Ok(result)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
   Cette fonction analyse la structure du projet et retourne un arbre de dossiers et de fichiers.
*/
fn get_project_structure(path: String) -> Result<TreeNode, String> {
    let project_path = Path::new(&path);

    fn scan_tree(dir: &Path, base_path: &Path) -> Result<TreeNode, String> {
        let name = dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let relative_path = dir
            .strip_prefix(base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "".to_string());

        let mut node = TreeNode {
            name: if name.is_empty() {
                base_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Project")
                    .to_string()
            } else {
                name
            },
            path: relative_path,
            is_dir: true,
            size: 0,
            children: Vec::new(),
        };
        let mut total_size: u64 = 0;

        if let Ok(entries) = fs::read_dir(dir) {
            let mut entries_vec: Vec<_> = entries.filter_map(|e| e.ok()).collect();

            // Trier: dossiers d'abord, puis fichiers, alphabétiquement
            entries_vec.sort_by(|a, b| {
                let a_is_dir = a.path().is_dir();
                let b_is_dir = b.path().is_dir();

                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => {
                        let a_name = a.file_name().to_string_lossy().to_lowercase();
                        let b_name = b.file_name().to_string_lossy().to_lowercase();
                        a_name.cmp(&b_name)
                    }
                }
            });

            for entry in entries_vec {
                let path = entry.path();

                // Ignorer les dossiers à exclure
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name == ".git"
                        || name == "node_modules"
                        || name == "target"
                        || name == "dist"
                        || name == "build"
                        || name == "out"
                    {
                        continue;
                    }
                }

                if path.is_dir() {
                    if let Ok(child_node) = scan_tree(&path, base_path) {
                        total_size += child_node.size;
                        node.children.push(child_node);
                    }
                } else if path.is_file() {
                    let file_size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();

                    let file_path = path
                        .strip_prefix(base_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| file_name.clone());

                    node.children.push(TreeNode {
                        name: file_name,
                        path: file_path,
                        is_dir: false,
                        size: file_size,
                        children: Vec::new(),
                    });
                    total_size += file_size;
                }
            }
        }
        node.size = total_size;

        Ok(node)
    }

    scan_tree(project_path, project_path)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
   Cette fonction lit le contenu d'un fichier et le retourne sous forme de chaîne de caractères.
*/
fn read_file_content(path: String) -> Result<String, String> {

    // Chemin du fichier à lire
    let file_path = Path::new(&path);

    // Vérifier que c'est bien un fichier
    if !file_path.is_file() {
        return Err("Le chemin ne correspond pas à un fichier".to_string());
    }

    // Lire les bytes bruts du fichier
    let mut file =
        fs::File::open(file_path).map_err(|e| format!("Erreur d'accès au fichier: {}", e))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Erreur de lecture: {}", e))?;

    fn decode_content(buffer: &[u8]) -> String {
        let (decoded, _, had_errors) = UTF_8.decode(buffer);
        if !had_errors {
            return decoded.into_owned();
        }
        WINDOWS_1252.decode(buffer).0.into_owned()
    }

    // Limiter la taille pour éviter de surcharger l'UI (max 1MB)
    if buffer.len() > 1_000_000 {
        let preview = decode_content(&buffer[..1000.min(buffer.len())]);
        return Ok(format!(
            "[Fichier trop volumineux pour être affiché]\n\nTaille: {} octets\n\nAperçu des 1000 premiers caractères:\n\n{}",
            buffer.len(),
            preview
        ));
    }

    Ok(decode_content(&buffer))
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
   Cette fonction lit un fichier et retourne son contenu encodé en base64.
*/
fn read_file_as_base64(path: String) -> Result<String, String> {
    use std::io::Read;

    let file_path = Path::new(&path);

    // Vérifier que le fichier existe
    if !file_path.exists() {
        return Err("Le fichier n'existe pas".to_string());
    }

    // Vérifier que c'est bien un fichier
    if !file_path.is_file() {
        return Err("Le chemin ne correspond pas à un fichier".to_string());
    }

    // Ouvrir le fichier
    let mut file = fs::File::open(file_path)
        .map_err(|e| format!("Erreur lors de l'ouverture du fichier: {}", e))?;

    // Lire le contenu en bytes
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Erreur lors de la lecture du fichier: {}", e))?;

    // Encoder en base64
    use base64::{engine::general_purpose, Engine as _};
    let base64_string = general_purpose::STANDARD.encode(&buffer);

    Ok(base64_string)
}

/* 
    Cette fonction scanne récursivement les fichiers d'un répertoire pour calculer la taille totale et le nombre de fichiers.
    Elle ignore certains dossiers communs comme .git, node_modules, target, dist, build et out.
*/
fn scan_files_recursive(dir: &Path, total_size: &mut u64, file_count: &mut u64) -> Result<(), String> {

    // Lire les entrées du répertoire
    if let Ok(entries) = fs::read_dir(dir) {
        
        // Parcourir les entrées du répertoire
        for entry in entries.flatten() {

            // Obtenir le chemin de l'entrée
            let path = entry.path();

            // Ignorer les dossiers communs à exclure
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name == ".git"
                    || name == "node_modules"
                    || name == "target"
                    || name == "dist"
                    || name == "build"
                    || name == "out"
                {
                    continue;
                }
            }

            // Si c'est un dossier, on continue la récursion
            if path.is_dir() {
                scan_files_recursive(&path, total_size, file_count)?; // Appel récursif pour les sous-dossiers
            }
            // Si c'est un fichier, on ajoute sa taille et on incrémente le compteur 
            else if path.is_file() {
                // Obtenir les métadonnées du fichier pour récupérer sa taille
                if let Ok(metadata) = fs::metadata(&path) {
                    *total_size += metadata.len();
                    *file_count += 1;
                }
            }

        }
    }

    Ok(())
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction calcule la taille moyenne des fichiers dans un répertoire.
*/
fn calculate_avg_file_size(path: String) -> Result<f64, String> {

    let project_path = Path::new(&path); // Créer un objet Path à partir du chemin fourni
    let mut total_size: u64 = 0;                // Taille totale des fichiers
    let mut file_count: u64 = 0;                // Compteur de fichiers

    // Parcourir les fichiers du projet pour calculer la taille totale et le nombre de fichiers
    scan_files_recursive(project_path, &mut total_size, &mut file_count)
        .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Si aucun fichier n'est trouvé, retourner 0.0 pour éviter la division par zéro
    if file_count == 0 {
        return Ok(0.0);
    }

    Ok(total_size as f64 / file_count as f64) // Calculer la taille moyenne des fichiers
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction retourne la palette de couleurs utilisée dans l'application.
*/
fn get_colors() -> HashMap<String, ColorPalette> {
    get_color_palette()
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction calcule la taille d'un dossier de manière récursive.
    La récursion permet de parcourir tous les sous-dossiers et fichiers pour obtenir la taille totale sans répétition.
*/
fn get_folder_size(path: &Path) -> u64 {
    let mut size = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                size += get_folder_size(&path);
            } else if path.is_file() {
                if let Ok(metadata) = fs::metadata(&path) {
                    size += metadata.len();
                }
            }
        }
    }
    size
}

// Extensions de fichiers de code
/*
    Cette constante contient les extensions de fichiers considérées comme des fichiers de code source.
    Elle est utilisée pour identifier et compter les fichiers de code lors de l'analyse du projet.
*/
const CODE_EXTENSIONS: &[&str] = &[
    "c", "h", "cpp", "hpp", "cc", "hh", "cxx", "hxx", "rs", "go", "py", "java", "js", "ts", "jsx",
    "tsx", "cs", "php", "rb", "swift", "kt", "scala", "r", "sh", "bash", "ps1", "bat", "cmd",
    "html", "css", "scss", "sass", "less", "json", "xml", "yaml", "yml", "toml", "ini", "cfg",
    "config", "sql", "lua", "vim", "md", "svelte", "vue", "dart",
];

// Extensions de fichiers ressources
/*
    Cette constante contient les extensions de fichiers considérées comme des ressources.
    Elle est utilisée pour identifier et compter les fichiers de ressources lors de l'analyse du projet.
*/
const RESOURCE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "svg", "webp", "ico", "mp3", "wav", "ogg", "flac", "m4a",
    "aac", "mp4", "avi", "mov", "mkv", "webm", "flv", "ttf", "otf", "woff", "woff2", "eot", "pdf",
    "doc", "docx", "xls", "xlsx", "ppt", "pptx",
];

// Extensions de fichiers exécutables
/*
    Cette constante contient les extensions de fichiers considérées comme des fichiers exécutables.
    Elle est utilisée pour identifier et compter les fichiers exécutables lors de l'analyse du projet.
*/
const EXECUTABLE_EXTENSIONS: &[&str] = &["exe", "dll", "so", "dylib", "app", "apk", "jar"];

// Dossiers à ignorer lors du scan
/*
    Cette constante contient les noms de dossiers à ignorer lors du scan du projet.
    Elle est utilisée pour éviter de scanner les dossiers non pertinents.
*/
const IGNORED_DIRS: &[&str] = &[
    "node_modules",
    "target",
    ".git",
    "build",
    "dist",
    "out",
    ".next",
];

// Structure pour stocker les statistiques de fichiers
/*
    Cette structure contient les statistiques des fichiers analysés.
    Elle est utilisée pour stocker les compteurs de fichiers par type et d'autres métriques.
*/
struct FileStats {
    code_files: u32,
    resource_files: u32,
    executable_files: u32,
    total_files: u32,
    total_lines: u64,
    max_depth: u32,
    largest_file_path: String,
    largest_file_size: u64,
    extension_counts: HashMap<String, u32>,
    language_bytes: HashMap<String, u64>,
    total_depth: u64,
    depth_count: u64,
}

/*
    Cette fonction scanne un répertoire de manière récursive pour collecter des statistiques sur les fichiers.
    Elle met à jour les compteurs de fichiers de code, ressources, exécutables, le nombre total de fichiers,
    le nombre total de lignes de code, la profondeur maximale et d'autres métriques comme le fichier le plus volumineux,
    la répartition par extension et par langage, et la profondeur moyenne des fichiers.
*/
fn scan_directory_for_stats(
    dir: &Path,
    current_depth: u32,
    stats: &mut FileStats,
    extensions_map: &HashMap<&str, &str>,
) -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if IGNORED_DIRS.contains(&dir_name) {
                    continue;
                }
                scan_directory_for_stats(&path, current_depth + 1, stats, extensions_map)?;
            } else if path.is_file() {
                stats.total_files += 1;
                stats.total_depth += current_depth as u64;
                stats.depth_count += 1;

                if let Ok(metadata) = fs::metadata(&path) {
                    let file_size = metadata.len();
                    if file_size > stats.largest_file_size {
                        stats.largest_file_size = file_size;
                        stats.largest_file_path = path
                            .strip_prefix("./")
                            .or_else(|_| path.strip_prefix(dir).map_err(|e| e))
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| path.to_string_lossy().to_string());
                    }

                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        let ext_lower = ext.to_lowercase();
                        *stats.extension_counts.entry(ext_lower.clone()).or_insert(0) += 1;

                        if let Some(&language) = extensions_map.get(ext_lower.as_str()) {
                            *stats
                                .language_bytes
                                .entry(language.to_string())
                                .or_insert(0) += file_size;
                        }

                        if CODE_EXTENSIONS.contains(&ext_lower.as_str()) {
                            stats.code_files += 1;

                            // Compter les lignes de code
                            if let Ok(content) = fs::read_to_string(&path) {
                                stats.total_lines += content.lines().count() as u64;
                            }
                        } else if RESOURCE_EXTENSIONS.contains(&ext_lower.as_str()) {
                            stats.resource_files += 1;
                        } else if EXECUTABLE_EXTENSIONS.contains(&ext_lower.as_str()) {
                            stats.executable_files += 1;
                        }
                    }
                }
            }
        }
    }

    // Mettre à jour la profondeur maximale
    if current_depth > stats.max_depth {
        stats.max_depth = current_depth;
    }

    Ok(())
}

/*
    Cette fonction scanne les fichiers de configuration d'éditeurs pour détecter l'éditeur utilisé.
*/
fn scan_for_editor_files(dir: &Path, editors: &mut HashMap<String, u32>) -> std::io::Result<()> {
    // Lire les entrées du répertoire
    if let Ok(entries) = fs::read_dir(dir) {
        // Parcourir les entrées du répertoire
        for entry in entries.flatten() {
            let path = entry.path(); // Récupérer le chemin du fichier ou dossier

            // Ignorer les dossiers communs à exclure
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Vérifier les dossiers de configuration d'éditeurs
                match dir_name {
                    ".vscode" => {
                        *editors.entry("VS Code".to_string()).or_insert(0) += 5;
                    }
                    ".idea" => {
                        *editors.entry("IntelliJ IDEA".to_string()).or_insert(0) += 5;
                    }
                    _ => {}
                }

                if !IGNORED_DIRS.contains(&dir_name) {
                    scan_for_editor_files(&path, editors)?;
                }
            }
            // Vérifier les fichiers de configuration d'éditeurs
            else if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    match file_name {
                        "settings.json" | "launch.json" | "tasks.json" => {
                            *editors.entry("VS Code".to_string()).or_insert(0) += 1;
                            // Augmenter le compteur pour VS Code
                        }
                        ".sublime-project" | ".sublime-workspace" => {
                            *editors.entry("Sublime Text".to_string()).or_insert(0) += 1;
                            // Augmenter le compteur pour Sublime Text
                        }
                        ".project" | ".classpath" => {
                            *editors.entry("Eclipse".to_string()).or_insert(0) += 1;
                            // Augmenter le compteur pour Eclipse
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

/*
    Cette fonction détecte l'éditeur utilisé dans le projet en scannant les fichiers de configuration d'éditeurs.
    Elle retourne le nom de l'éditeur le plus probable ou "Inconnu" si aucun éditeur n'est détecté.
*/
fn detect_editor(dir: &Path) -> String {
    let mut editor_files: HashMap<String, u32> = HashMap::new();

    // Add error handling here
    let _ = scan_for_editor_files(dir, &mut editor_files);

    editor_files
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(editor, _)| editor.clone())
        .unwrap_or_else(|| "Inconnu".to_string())
}

/*
    Cette fonction calcule la complexité du projet avec une analyse détaillée.
*/
fn calculate_complexity(project_path: &Path) -> String {
    match complex::calculate_project_complexity(project_path) {
        Ok(complexity) => complex::format_complexity(&complexity),
        Err(_) => "Indéterminée • O(?)".to_string(),
    }
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction retourne les informations générales du projet, telles que :
    • la taille
    • la complexité
    • le nombre de fichiers de code, de ressources et d'exécutables
    • l'éditeur détecté. (parmis les choix disponibles: VS Code, IntelliJ IDEA, Sublime Text, Eclipse, Inconnu)
*/
async fn get_project_info(path: String) -> Result<ProjectInfo, String> {
    let project_path = Path::new(&path);
    if !project_path.exists() {
        return Err("Le chemin n'existe pas".to_string());
    }

    // Initialiser les statistiques
    let mut stats = FileStats {
        code_files: 0,
        resource_files: 0,
        executable_files: 0,
        total_files: 0,
        total_lines: 0,
        max_depth: 0,
        largest_file_path: String::new(),
        largest_file_size: 0,
        extension_counts: HashMap::new(),
        language_bytes: HashMap::new(),
        total_depth: 0,
        depth_count: 0,
    };

    // Scanner le projet pour collecter les statistiques
    let extensions_map: HashMap<&str, &str> = EXTENSIONS.iter().cloned().collect();
    scan_directory_for_stats(project_path, 0, &mut stats, &extensions_map)
        .map_err(|e| e.to_string())?;

    // Calculer la taille du projet
    let size = get_folder_size(project_path);

    // Déterminer la complexité avec analyse complète
    let complexity = calculate_complexity(project_path);

    // Détecter l'éditeur utilisé
    let detected_editor = detect_editor(project_path);

    let code_files_percentage = if stats.total_files > 0 {
        (stats.code_files as f64 / stats.total_files as f64) * 100.0
    } else {
        0.0
    };

    let dominant_language = stats
        .language_bytes
        .iter()
        .max_by_key(|(_, &bytes)| bytes)
        .map(|(language, _)| language.clone())
        .unwrap_or_else(|| "Inconnu".to_string());

    let dominant_extension = stats
        .extension_counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(extension, _)| extension.clone())
        .unwrap_or_else(|| "Inconnu".to_string());

    let average_depth = if stats.depth_count > 0 {
        stats.total_depth as f64 / stats.depth_count as f64
    } else {
        0.0
    };

    Ok(ProjectInfo {
        path: path.clone(),
        size,
        complexity,
        code_files_count: stats.code_files,
        resource_files_count: stats.resource_files,
        executable_count: stats.executable_files,
        total_files_count: stats.total_files,
        code_files_percentage,
        largest_file_path: if stats.largest_file_path.is_empty() {
            "Aucun fichier".to_string()
        } else {
            stats.largest_file_path.clone()
        },
        largest_file_size: stats.largest_file_size,
        dominant_language,
        dominant_extension,
        average_depth,
        max_depth: stats.max_depth,
        detected_editor,
    })
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction calcule la complexité logicielle du projet en utilisant une analyse détaillée.
    Elle retourne un objet ProjectComplexity contenant les métriques de complexité logicielle.
*/
async fn get_project_complexity(path: String) -> Result<complex::ProjectComplexity, String> {
    let project_path = Path::new(&path);
    if !project_path.exists() {
        return Err("Le chemin n'existe pas".to_string());
    }

    complex::calculate_project_complexity(project_path)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient l'historique complet du projet, y compris les changements Git et locaux.
    Elle retourne un objet ProjectHistory contenant toutes les informations d'historique.
*/
async fn get_project_history(path: String) -> Result<ProjectHistory, String> {
    let project_path = Path::new(&path);
    let has_git = is_git_repository(project_path);

    let git_history = if has_git {
        get_git_history(project_path, 100).ok()
    } else {
        None
    };

    let git_stats = if has_git {
        get_git_stats(project_path).ok()
    } else {
        None
    };

    let uncommitted_changes = if has_git {
        get_uncommitted_changes(project_path).ok()
    } else {
        None
    };

    let local_history = get_local_history(project_path).map_err(|e| {
        format!(
            "Erreur lors de la récupération de l'historique local: {}",
            e
        )
    })?;
    let recent_local_changes = detect_local_changes(project_path)
        .map_err(|e| format!("Erreur lors de la détection des changements locaux: {}", e))?;

    Ok(ProjectHistory {
        has_git,
        git_history,
        git_stats,
        uncommitted_changes,
        local_history,
        recent_local_changes,
    })
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient uniquement l'historique Git du projet.
    Elle retourne un objet GitHistory contenant les informations d'historique Git.
*/
async fn get_git_project_history(path: String) -> Result<Option<GitHistory>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_git_history(project_path, 100).ok())
    } else {
        Ok(None)
    }
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient uniquement l'historique Git du projet.
    Elle retourne un objet GitHistory contenant les informations d'historique Git.
*/
async fn get_git_project_stats(path: String) -> Result<Option<GitStats>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_git_stats(project_path).ok())
    } else {
        Ok(None)
    }
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient les changements non commités.
    Elle retourne un vecteur de chaînes de caractères représentant les fichiers modifiés.
*/
async fn get_git_uncommitted_changes(path: String) -> Result<Option<Vec<String>>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_uncommitted_changes(project_path).ok())
    } else {
        Ok(None)
    }
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient l'historique local du projet.
    Elle retourne un objet LocalHistory contenant les informations d'historique local.
*/
async fn get_local_project_history(path: String) -> Result<LocalHistory, String> {
    let project_path = Path::new(&path);
    get_local_history(project_path)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction enregistre un événement de scan.
    Elle ne retourne rien en cas de succès.
*/
async fn record_scan_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "scan", "Project scanned", Vec::new())?;
    Ok(())
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction enregistre un événement d'analyse.
    Elle ne retourne rien en cas de succès.
*/
async fn record_analysis_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "analysis", "Project analyzed", Vec::new())?;
    Ok(())
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction enregistre un événement de modification.
    Elle ne retourne rien en cas de succès.
*/
async fn record_modification_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "modification", "Project modified", Vec::new())?;
    Ok(())
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction obtient les changements locaux récents.
    Elle retourne un vecteur de chaînes de caractères représentant les fichiers modifiés.
*/
async fn get_recent_local_changes(path: String) -> Result<Vec<String>, String> {
    let project_path = Path::new(&path);
    detect_local_changes(project_path)
}

// Note : tauri::command est un attribut qui permet d'exposer une fonction Rust à l'interface JavaScript/TypeScript de l'application Tauri.
#[tauri::command]
/*
    Cette fonction vérifie si le projet est un dépôt Git.
    Elle retourne un booléen indiquant si le projet est un dépôt Git.
*/
async fn check_is_git_repository(path: String) -> Result<bool, String> {
    let project_path = Path::new(&path);
    Ok(is_git_repository(project_path))
}

// cfg_attr est un attribut conditionnel qui permet d'appliquer des attributs à une fonction en fonction de la configuration de compilation.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
/*
    Cette fonction est le point d'entrée de l'application Tauri.
    Elle configure et lance l'application avec les plugins et les commandes définies.
*/
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_project,
            back_to_dashboard,
            analyze_project_languages,
            analyze_project_content,
            analyze_project_modifications,
            analyze_modified_files,
            get_project_structure,
            read_file_content,
            read_file_as_base64,
            calculate_avg_file_size,
            get_colors,
            get_project_info,
            get_project_complexity,
            // Nouvelles commandes pour l'historique
            get_project_history,
            get_git_project_history,
            get_git_project_stats,
            get_git_uncommitted_changes,
            get_local_project_history,
            record_scan_event,
            record_analysis_event,
            record_modification_event,
            get_recent_local_changes,
            check_is_git_repository,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
