// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod colors;
mod complex;
mod history;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use colors::{ColorPalette, get_color_palette};
use history::{
    get_git_history, get_git_stats, get_uncommitted_changes, is_git_repository,
    get_local_history, add_local_event, detect_local_changes,
    GitHistory, GitStats, LocalHistory
};

#[derive(Serialize, Deserialize)]
struct Modification {
    date: String,
    bytes: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct FileTypeCount {
    extension: String,
    count: u32,
}

#[derive(Serialize, Deserialize)]
struct ProjectContent {
    category: String,
    files: Vec<FileTypeCount>,
    total: u32,
}

#[derive(Serialize, Deserialize)]
struct ModifiedFile {
    file: String,
    modifications: u64,
}

#[derive(Serialize, Deserialize, Clone)]
struct TreeNode {
    name: String,
    path: String,
    is_dir: bool,
    children: Vec<TreeNode>,
}

#[derive(Serialize, Deserialize)]
struct ProjectInfo {
    path: String,
    size: u64,
    complexity: String,
    code_files_count: u32,
    resource_files_count: u32,
    executable_count: u32,
    total_files_count: u32,
    max_depth: u32,
    detected_editor: String,
}

/// Structure pour l'historique complet du projet
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectHistory {
    pub has_git: bool,
    pub git_history: Option<GitHistory>,
    pub git_stats: Option<GitStats>,
    pub uncommitted_changes: Option<Vec<String>>,
    pub local_history: LocalHistory,
    pub recent_local_changes: Vec<String>,
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
    window.center().ok();

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
    window.center().ok();

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

            if path.is_dir() {
                scan_directory(&path, language_stats, total_bytes, extensions_map)?;
            } else if path.is_file() {
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

/* Analyse les langages du projet */
#[tauri::command]
async fn analyze_project_languages(path: String) -> Result<HashMap<String, f64>, String> {
    let project_path = Path::new(&path);
    
    if !project_path.exists() {
        return Err("Le chemin du projet n'existe pas".to_string());
    }

    let mut language_stats: HashMap<String, u64> = HashMap::new();
    let mut total_bytes: u64 = 0;

    let extensions_map: HashMap<&str, &str> = EXTENSIONS.iter().cloned().collect();

    // Parcourir les fichiers du projet
    scan_directory(project_path, &mut language_stats, &mut total_bytes, &extensions_map)
        .map_err(|e| format!("Erreur lors du scan: {}", e))?;

    // Convertir en pourcentages
    let mut percentages: HashMap<String, f64> = HashMap::new();
    if total_bytes > 0 {
        for (language, bytes) in language_stats {
            let percentage = (bytes as f64 / total_bytes as f64) * 100.0;
            percentages.insert(language, (percentage * 100.0).round() / 100.0); // Arrondir à 2 décimales
        }
    }

    Ok(percentages)
}

/* Analyse le contenu détaillé du projet par catégories et extensions */
#[tauri::command]
async fn analyze_project_content(path: String) -> Result<Vec<ProjectContent>, String> {
    let project_path = Path::new(&path);
    
    if !project_path.exists() {
        return Err("Le chemin du projet n'existe pas".to_string());
    }

    let mut extension_counts: HashMap<String, u32> = HashMap::new();

    // Parcourir tous les fichiers et compter par extension
    fn scan_extensions(
        dir: &Path,
        counts: &mut HashMap<String, u32>,
    ) -> Result<(), String> {
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
        
        categories.entry(category.to_string())
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

/* Analyse les modifications du projet */
#[tauri::command]
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

/* Analyse les fichiers les plus modifiés */
#[tauri::command]
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

/* Analyse la structure du projet */
#[tauri::command]
fn get_project_structure(path: String) -> Result<TreeNode, String> {
    let project_path = Path::new(&path);
    
    fn scan_tree(
        dir: &Path,
        base_path: &Path,
    ) -> Result<TreeNode, String> {
        let name = dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        let relative_path = dir.strip_prefix(base_path)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "".to_string());
        
        let mut node = TreeNode {
            name: if name.is_empty() { 
                base_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Project")
                    .to_string() 
            } else { 
                name 
            },
            path: relative_path,
            is_dir: true,
            children: Vec::new(),
        };
        
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
                        node.children.push(child_node);
                    }
                } else if path.is_file() {
                    let file_name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let file_path = path.strip_prefix(base_path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| file_name.clone());
                    
                    node.children.push(TreeNode {
                        name: file_name,
                        path: file_path,
                        is_dir: false,
                        children: Vec::new(),
                    });
                }
            }
        }
        
        Ok(node)
    }
    
    scan_tree(project_path, project_path)
}

/* Lit le contenu d'un fichier */
#[tauri::command]
fn read_file_content(path: String) -> Result<String, String> {
    use std::io::Read;
    
    let file_path = Path::new(&path);
    
    // Vérifier que c'est bien un fichier
    if !file_path.is_file() {
        return Err("Le chemin ne correspond pas à un fichier".to_string());
    }
    
    // Lire les bytes bruts du fichier
    let mut file = fs::File::open(file_path)
        .map_err(|e| format!("Erreur d'accès au fichier: {}", e))?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Erreur de lecture: {}", e))?;
    
    // Limiter la taille pour éviter de surcharger l'UI (max 1MB)
    if buffer.len() > 1_000_000 {
        // Essayer de convertir les 1000 premiers caractères
        let preview = String::from_utf8_lossy(&buffer[..1000.min(buffer.len())]);
        return Ok(format!(
            "[Fichier trop volumineux pour être affiché]\n\nTaille: {} octets\n\nAperçu des 1000 premiers caractères:\n\n{}",
            buffer.len(),
            preview
        ));
    }
    
    // Essayer de convertir en UTF-8, en remplaçant les caractères invalides
    match String::from_utf8(buffer.clone()) {
        Ok(content) => Ok(content),
        Err(_) => {
            // Si UTF-8 échoue, utiliser from_utf8_lossy qui remplace les caractères invalides
            let content = String::from_utf8_lossy(&buffer);
            Ok(content.to_string())
        }
    }
}

/* Lit un fichier et retourne son contenu encodé en base64 */
#[tauri::command]
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
    use base64::{Engine as _, engine::general_purpose};
    let base64_string = general_purpose::STANDARD.encode(&buffer);
    
    Ok(base64_string)
}


/* Récupère la palette de couleurs unifiée */
#[tauri::command]
fn get_colors() -> HashMap<String, ColorPalette> {
    get_color_palette()
}

/* Calcule la taille d'un dossier de manière récursive */
#[tauri::command]
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

/* Extensions de fichiers de code */
const CODE_EXTENSIONS: &[&str] = &[
    "c", "h", "cpp", "hpp", "cc", "hh", "cxx", "hxx",
    "rs", "go", "py", "java", "js", "ts", "jsx", "tsx",
    "cs", "php", "rb", "swift", "kt", "scala", "r",
    "sh", "bash", "ps1", "bat", "cmd",
    "html", "css", "scss", "sass", "less",
    "json", "xml", "yaml", "yml", "toml", "ini", "cfg", "config",
    "sql", "lua", "vim", "md", "svelte", "vue", "dart"
];

/* Extensions de fichiers ressources */
const RESOURCE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "svg", "webp", "ico",
    "mp3", "wav", "ogg", "flac", "m4a", "aac",
    "mp4", "avi", "mov", "mkv", "webm", "flv",
    "ttf", "otf", "woff", "woff2", "eot",
    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx"
];

/* Extensions de fichiers exécutables */
const EXECUTABLE_EXTENSIONS: &[&str] = &[
    "exe", "dll", "so", "dylib", "app", "apk", "jar"
];

/* Dossiers à ignorer lors du scan */
const IGNORED_DIRS: &[&str] = &[
    "node_modules", "target", ".git", "build", "dist", "out", ".next"
];

struct FileStats {
    code_files: u32,
    resource_files: u32,
    executable_files: u32,
    total_files: u32,
    total_lines: u64,
    max_depth: u32,
}

/* Scanne un dossier et compte les fichiers par type */
fn scan_directory_for_stats(dir: &Path, current_depth: u32, stats: &mut FileStats) -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if IGNORED_DIRS.contains(&dir_name) {
                    continue;
                }
                scan_directory_for_stats(&path, current_depth + 1, stats)?;
            } else if path.is_file() {
                stats.total_files += 1;
                
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    
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
    
    // Mettre à jour la profondeur maximale
    if current_depth > stats.max_depth {
        stats.max_depth = current_depth;
    }
    
    Ok(())
}


/// Scanne les fichiers de configuration pour détecter l'éditeur utilisé
fn scan_for_editor_files(dir: &Path, editors: &mut HashMap<String, u32>) -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            
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
            } else if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    match file_name {
                        "settings.json" | "launch.json" | "tasks.json" => {
                            *editors.entry("VS Code".to_string()).or_insert(0) += 1;
                        }
                        ".sublime-project" | ".sublime-workspace" => {
                            *editors.entry("Sublime Text".to_string()).or_insert(0) += 1;
                        }
                        ".project" | ".classpath" => {
                            *editors.entry("Eclipse".to_string()).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}

/* Détecte l'éditeur utilisé basé sur les fichiers de configuration */
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

/* Détermine la complexité du projet avec analyse détaillée */
fn calculate_complexity(project_path: &Path) -> String {
    match complex::calculate_project_complexity(project_path) {
        Ok(complexity) => complex::format_complexity(&complexity),
        Err(_) => "Indéterminée • O(?)".to_string()
    }
}

/* Obtient les informations complètes du projet */
#[tauri::command]
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
    };

    // Scanner le projet pour collecter les statistiques
    scan_directory_for_stats(project_path, 0, &mut stats).map_err(|e| e.to_string())?;

    // Calculer la taille du projet
    let size = get_folder_size(project_path);

    // Déterminer la complexité avec analyse complète
    let complexity = calculate_complexity(project_path);

    // Détecter l'éditeur utilisé
    let detected_editor = detect_editor(project_path);

    Ok(ProjectInfo {
        path: path.clone(),
        size,
        complexity,
        code_files_count: stats.code_files,
        resource_files_count: stats.resource_files,
        executable_count: stats.executable_files,
        total_files_count: stats.total_files,
        max_depth: stats.max_depth,
        detected_editor,
    })
}

/* Obtient les détails complets de la complexité du projet */
#[tauri::command]
async fn get_project_complexity(path: String) -> Result<complex::ProjectComplexity, String> {
    let project_path = Path::new(&path);
    if !project_path.exists() {
        return Err("Le chemin n'existe pas".to_string());
    }

    complex::calculate_project_complexity(project_path)
}

/* Obtient l'historique complet du projet */
#[tauri::command]
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
    
    let local_history = get_local_history(project_path)
        .map_err(|e| format!("Erreur lors de la récupération de l'historique local: {}", e))?;
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

/* Obtient uniquement l'historique Git */
#[tauri::command]
async fn get_git_project_history(path: String) -> Result<Option<GitHistory>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_git_history(project_path, 100).ok())
    } else {
        Ok(None)
    }
}

/* Obtient les statistiques Git */
#[tauri::command]
async fn get_git_project_stats(path: String) -> Result<Option<GitStats>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_git_stats(project_path).ok())
    } else {
        Ok(None)
    }
}

/* Obtient les changements non commités */
#[tauri::command]
async fn get_git_uncommitted_changes(path: String) -> Result<Option<Vec<String>>, String> {
    let project_path = Path::new(&path);
    if is_git_repository(project_path) {
        Ok(get_uncommitted_changes(project_path).ok())
    } else {
        Ok(None)
    }
}

/* Obtient l'historique local */
#[tauri::command]
async fn get_local_project_history(path: String) -> Result<LocalHistory, String> {
    let project_path = Path::new(&path);
    get_local_history(project_path)
}

/* Enregistre un événement de scan */
#[tauri::command]
async fn record_scan_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "scan", "Project scanned", Vec::new())?;
    Ok(())
}

/* Enregistre un événement d'analyse */
#[tauri::command]
async fn record_analysis_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "analysis", "Project analyzed", Vec::new())?;
    Ok(())
}

/* Enregistre un événement de modification */
#[tauri::command]
async fn record_modification_event(path: String) -> Result<(), String> {
    let project_path = Path::new(&path);
    add_local_event(project_path, "modification", "Project modified", Vec::new())?;
    Ok(())
}

/* Obtient les changements locaux récents */
#[tauri::command]
async fn get_recent_local_changes(path: String) -> Result<Vec<String>, String> {
    let project_path = Path::new(&path);
    detect_local_changes(project_path)
}

/* Vérifie si le projet est un dépôt Git */
#[tauri::command]
async fn check_is_git_repository(path: String) -> Result<bool, String> {
    let project_path = Path::new(&path);
    Ok(is_git_repository(project_path))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
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
