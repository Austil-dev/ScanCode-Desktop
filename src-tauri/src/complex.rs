use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// derive est un attribut qui permet de générer les implémentations pour FileComplexity pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Cette structure contient toutes les métriques de complexité pour un fichier de code.
    - path : chemin du fichier
    - cyclomatic : complexité cyclomatique (McCabe)
    - npath : complexité NPath (nombre de chemins d'exécution)
    - loc_phy : lignes de code physiques
    - loc_pro : lignes de code exécutables
    - halstead_volume : volume d'Halstead
    - maintainability_index : index de maintenabilité
*/
pub struct FileComplexity {
    pub path: String,
    pub cyclomatic: u32,
    pub npath: u64,
    pub loc_phy: u32,
    pub loc_pro: u32,
    pub halstead_volume: f64,
    pub maintainability_index: f64,
}

// derive est un attribut qui permet de générer les implémentations pour FileComplexity pour les traits Debug, Clone, Serialize et Deserialize.
#[derive(Debug, Clone, Serialize, Deserialize)]
/*
    Cette structure contient les métriques de complexité globales pour un projet.
    - level : niveau global de complexité (0-10)
    - software_complexity : classification de la complexité logicielle ("Très faible", "faible", ...)
    - total_cyclomatic : somme des complexités cyclomatiques de tous les fichiers
    - avg_cyclomatic : moyenne des complexités cyclomatiques
    - total_npath : somme des complexités NPath de tous les fichiers
    - total_loc : somme des lignes de code exécutables de tous les fichiers
    - avg_maintainability : moyenne des index de maintenabilité
    - files_analyzed : nombre total de fichiers analysés
*/
pub struct ProjectComplexity {
    pub level: f32,                  // "Acceptable", "Modérée", ...
    pub software_complexity: String, // "Très faible", "faible", ...
    pub total_cyclomatic: u32,
    pub avg_cyclomatic: f64,
    pub total_npath: u64,
    pub total_loc: u32,
    pub avg_maintainability: f64,
    pub files_analyzed: u32,
}

/*
    Calcule la complexité cyclomatique d'un fichier de code.
    Note : La complexité cyclomatique est une mesure du nombre de chemins linéairement indépendants dans un programme.
    Elle est calculée en comptant les structures de contrôle (if, for, while, case, etc.) et en ajoutant 1.
*/
pub fn calculate_cyclomatic_complexity(content: &str) -> u32 {
    let mut complexity = 1; // La complexité de base est 1

    let keywords = [
        "if", "else if", "for", "while", "loop", "match", "catch", "&&", "||", "?", "case", "when",
    ];

    for line in content.lines() {
        let trimmed = line.trim();

        // Ignorer les commentaires
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
            continue;
        }

        // Compter les mots-clés de contrôle
        for keyword in &keywords {
            if keyword.len() == 2 {
                // Opérateurs logiques
                complexity += line.matches(keyword).count() as u32;
            } else {
                // Utiliser des délimiteurs pour éviter les faux positifs
                let pattern = format!("{}(", keyword);
                complexity += line.matches(&pattern).count() as u32;

                // Pour les mots-clés sans parenthèses
                if *keyword == "else if" || *keyword == "loop" {
                    complexity += line.matches(keyword).count() as u32;
                }
            }
        }

        // Compter les branches dans match
        if trimmed.contains("=>") && !trimmed.starts_with("//") {
            complexity += 1;
        }
    }

    complexity
}

/*
    Calcule la complexité NPath (nombre de chemins d'exécution) d'un fichier de code.
    Note : Cette fonction compte le nombre de chemins possible des structures de contrôle.
    Approximation : multiplication des complexités des structures imbriquées
*/
pub fn calculate_npath_complexity(content: &str) -> u64 {
    let mut npath: u64 = 1;
    let mut branch_depth = 0;
    let mut current_branches = vec![1u64];

    for line in content.lines() {
        let trimmed = line.trim();

        // Ignorer les commentaires
        if trimmed.starts_with("//") || trimmed.starts_with("/*") {
            continue;
        }

        // Détecter les débuts de blocs conditionnels
        if trimmed.contains("if ") && trimmed.contains("{") {
            branch_depth += 1;
            current_branches.push(2); // if/else = 2 branches minimum
        } else if trimmed.contains("match ") && trimmed.contains("{") {
            branch_depth += 1;
            current_branches.push(3); // match a au minimum 3 branches
        } else if trimmed.contains("for ") || trimmed.contains("while ") {
            branch_depth += 1;
            current_branches.push(2); // boucle = 2 chemins (entrer ou non)
        }

        // Compter les branches dans un match
        if branch_depth > 0 && trimmed.contains("=>") {
            if let Some(last) = current_branches.last_mut() {
                *last += 1;
            }
        }

        // Détecter les fins de blocs
        if trimmed == "}" && branch_depth > 0 {
            if let Some(branches) = current_branches.pop() {
                npath *= branches;
                branch_depth -= 1;
            }
        }
    }

    // Multiplier les branches restantes
    for branches in current_branches.iter().skip(1) {
        npath *= branches;
    }

    npath.max(1)
}

/*
    Calcule les métriques de lignes de code (LOC).
    Note : Calcule simplement le nombre de lignes non vides.
    Retourne (LOCphy, LOCpro) où :
    - LOCphy = nombre total de lignes physiques
    - LOCpro = nombre de lignes de code exécutable
*/
pub fn calculate_loc_metrics(content: &str) -> (u32, u32) {
    let mut loc_phy = 0u32;
    let mut loc_pro = 0u32;
    let mut in_multiline_comment = false;

    for line in content.lines() {
        loc_phy += 1;
        let trimmed = line.trim();

        // Gérer les commentaires multilignes
        if trimmed.contains("/*") {
            in_multiline_comment = true;
        }
        if trimmed.contains("*/") {
            in_multiline_comment = false;
            continue;
        }

        // Ignorer les lignes vides, commentaires et lignes purement décoratives
        if !trimmed.is_empty()
            && !trimmed.starts_with("//")
            && !in_multiline_comment
            && !trimmed.starts_with("*")
            && trimmed != "{"
            && trimmed != "}"
        {
            loc_pro += 1;
        }
    }

    (loc_phy, loc_pro)
}

/*
    Calcule le volume d'Halstead d'un fichier de code.
    Note : Cette fonction mesure la richesse de vocabulaire (opérandes) d'un programme.
    V = N * log2(n) où :
    - N = nombre total d'opérateurs et opérandes
    - n = nombre d'opérateurs et opérandes uniques
*/
pub fn calculate_halstead_volume(content: &str) -> f64 {
    let mut operators: HashMap<String, u32> = HashMap::new();
    let mut operands: HashMap<String, u32> = HashMap::new();

    // Liste des opérateurs courants
    let op_symbols = [
        "+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">=", "&&", "||", "!", "&", "|",
        "^", "<<", ">>", "+=", "-=", "*=", "/=", "++", "--", ".", "->", "::", "?",
    ];

    let keywords = [
        "if", "else", "for", "while", "loop", "match", "return", "break", "continue", "let", "mut",
        "fn", "struct", "enum", "impl", "trait", "use", "mod", "pub", "const", "static",
    ];

    for line in content.lines() {
        let trimmed = line.trim();

        // Ignorer les commentaires
        if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
            continue;
        }

        // Compter les opérateurs symboliques
        for op in &op_symbols {
            let count = line.matches(op).count() as u32;
            if count > 0 {
                *operators.entry(op.to_string()).or_insert(0) += count;
            }
        }

        // Compter les mots-clés (opérateurs)
        for keyword in &keywords {
            if line.contains(keyword) {
                *operators.entry(keyword.to_string()).or_insert(0) += 1;
            }
        }

        // Estimer les opérandes (identifiants)
        // On compte les mots qui ne sont pas des mots-clés
        let words: Vec<&str> = trimmed.split_whitespace().collect();
        for word in words {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            if !clean.is_empty()
                && !keywords.contains(&clean)
                && clean.chars().next().unwrap().is_alphabetic()
            {
                *operands.entry(clean.to_string()).or_insert(0) += 1;
            }
        }
    }

    let n1 = operators.len() as f64; // Nombre d'opérateurs uniques
    let n2 = operands.len() as f64; // Nombre d'opérandes uniques
    let total_n1: u32 = operators.values().sum(); // Total opérateurs
    let total_n2: u32 = operands.values().sum(); // Total opérandes

    let n = n1 + n2; // Nombre total d'opérateurs et opérandes uniques
    let total_n = (total_n1 + total_n2) as f64; // Nombre total d'opérateurs et opérandes

    // Calcul du volume d'Halstead
    if n > 0.0 {
        total_n * n.log2()
    } else {
        0.0
    }
}

/*
    Calcule l'index de maintenabilité d'un fichier de code.
    Note : L'index de maintenabilité est une mesure composite qui combine la complexité cyclomatique, le volume d'Halstead et les lignes de code.
    Il est normalisé entre 0 et 100, où 100 est le plus maintenable.
*/
pub fn calculate_maintainability_index(
    halstead_volume: f64,
    cyclomatic: u32,
    loc_pro: u32,
    loc_phy: u32,
) -> f64 {
    if loc_pro == 0 {
        return 100.0;
    }

    let v = halstead_volume.max(1.0);
    let g = cyclomatic as f64;
    let loc = loc_pro as f64;

    // Calcul de base de l'index de maintenabilité
    let mi = 171.0 - 5.2 * v.ln() - 0.23 * g - 16.2 * loc.ln();

    // Ajustement selon le pourcentage de commentaires
    let comment_lines = loc_phy.saturating_sub(loc_pro);
    let comment_percentage = if loc_phy > 0 {
        (comment_lines as f64 / loc_phy as f64) * 100.0
    } else {
        0.0
    };

    // Bonus si les commentaires sont entre 30% et 75%
    let comment_bonus = if (30.0..=75.0).contains(&comment_percentage) {
        5.0
    } else if comment_percentage < 30.0 {
        -2.0 // Pénalité pour manque de documentation
    } else {
        -1.0 // Pénalité légère pour sur-documentation
    };

    // Normaliser entre 0 et 100
    (mi + comment_bonus).max(0.0).min(100.0)
}

/*
    Analyse la complexité logicielle d'un fichier unique.
    Note : Cette fonction calcule plusieurs métriques de complexité pour un fichier de code donné.
*/
pub fn analyze_file_complexity(path: &Path) -> Result<FileComplexity, String> {
    // Lire le contenu du fichier
    let content = fs::read_to_string(path).map_err(|e| format!("Erreur lecture fichier: {}", e))?; // Gestion des erreurs de lecture du fichier

    // Calculer les métriques de complexité
    let cyclomatic = calculate_cyclomatic_complexity(&content);
    let npath = calculate_npath_complexity(&content);
    let (loc_phy, loc_pro) = calculate_loc_metrics(&content);
    let halstead_volume = calculate_halstead_volume(&content);
    let maintainability_index =
        calculate_maintainability_index(halstead_volume, cyclomatic, loc_pro, loc_phy);

    // Retourner les métriques dans une structure FileComplexity
    Ok(FileComplexity {
        path: path.to_string_lossy().to_string(),
        cyclomatic,
        npath,
        loc_phy,
        loc_pro,
        halstead_volume,
        maintainability_index,
    })
}

/*
    Liste des extensions de fichiers considérées comme des fichiers de code.
    Note : Cette liste peut être étendue pour inclure d'autres langages de programmation.
*/
const CODE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "jsx", "tsx", "java", "c", "cpp", "h", "hpp", "cs", "go", "php", "rb",
    "swift", "kt", "scala", "dart", "svelte",
];

/*
    Vérifie si un fichier est un fichier de code en fonction de son extension.
    Note : Cette fonction retourne true si l'extension du fichier est dans la liste CODE_EXTENSIONS.
*/
fn is_code_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        if let Some(ext_str) = ext.to_str() {
            return CODE_EXTENSIONS.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

/*
    Scanne récursivement un dossier et analyse tous les fichiers de code.
*/
fn scan_and_analyze_directory(
    dir: &Path,
    file_complexities: &mut Vec<FileComplexity>,
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
                    || name == ".vscode"
                    || name == ".idea"
                {
                    continue;
                }
            }

            if path.is_dir() {
                scan_and_analyze_directory(&path, file_complexities)?;
            } else if path.is_file() && is_code_file(&path) {
                // Analyser le fichier de code
                if let Ok(complexity) = analyze_file_complexity(&path) {
                    file_complexities.push(complexity);
                }
            }
        }
    }
    Ok(())
}

/*
    Arrondit une valeur f32 à 2 décimales.
*/
fn round(value: f32) -> f32 {
    value.round()
}

/*
    Détermine le niveau de complexité et renvoie le niveau et la classification associés à la complexité logicielle.
*/
fn determine_complexity_level(
    avg_cyclomatic: f64,
    avg_maintainability: f64,
    total_npath: u64,
) -> (f32, String) {
    // Niveau global destiné à l'utilisateur
    let level = if avg_cyclomatic <= 10.0 && avg_maintainability >= 70.0 {
        (avg_maintainability / avg_cyclomatic) as f32
    } else if avg_cyclomatic <= 20.0 && avg_maintainability >= 55.0 {
        (avg_maintainability / avg_cyclomatic) as f32
    } else if avg_cyclomatic <= 40.0 && avg_maintainability >= 35.0 {
        (avg_maintainability % avg_cyclomatic) as f32
    } else {
        (avg_maintainability % avg_cyclomatic) as f32
    };

    // Classification de la complexité logicielle
    let software_complexity =
        if avg_cyclomatic <= 5.0 && avg_maintainability >= 85.0 && total_npath < 100 {
            "Très faible"
        } else if avg_cyclomatic <= 10.0 && avg_maintainability >= 70.0 && total_npath < 1_000 {
            "Faible"
        } else if avg_cyclomatic <= 20.0 && avg_maintainability >= 55.0 && total_npath < 10_000 {
            "Modérée"
        } else if avg_cyclomatic <= 40.0 && avg_maintainability >= 35.0 {
            "Élevée"
        } else {
            "Critique"
        };

    (round(level), software_complexity.to_string())
}

/*
    Calcule la complexité logicielle globale du projet.
*/
pub fn calculate_project_complexity(project_path: &Path) -> Result<ProjectComplexity, String> {
    let mut file_complexities = Vec::new();

    // Scanner et analyser tous les fichiers de code
    scan_and_analyze_directory(project_path, &mut file_complexities)?;

    if file_complexities.is_empty() {
        return Ok(ProjectComplexity {
            level: 0.0,
            software_complexity: "Modérée".to_string(),
            total_cyclomatic: 0,
            avg_cyclomatic: 0.0,
            total_npath: 0,
            total_loc: 0,
            avg_maintainability: 100.0,
            files_analyzed: 0,
        });
    }

    // Agréger les métriques
    let total_cyclomatic: u32 = file_complexities.iter().map(|f| f.cyclomatic).sum();
    let total_npath: u64 = file_complexities.iter().map(|f| f.npath).sum();
    let total_loc: u32 = file_complexities.iter().map(|f| f.loc_pro).sum();
    let sum_maintainability: f64 = file_complexities
        .iter()
        .map(|f| f.maintainability_index)
        .sum();

    let files_analyzed = file_complexities.len() as u32;
    let avg_cyclomatic = total_cyclomatic as f64 / files_analyzed as f64;
    let avg_maintainability = sum_maintainability / files_analyzed as f64;

    // Déterminer le niveau et la notation
    let (level, software_complexity) =
        determine_complexity_level(avg_cyclomatic, avg_maintainability, total_npath);

    Ok(ProjectComplexity {
        level,
        software_complexity,
        total_cyclomatic,
        avg_cyclomatic,
        total_npath,
        total_loc,
        avg_maintainability,
        files_analyzed,
    })
}

/*
    Formate la complexité pour l'affichage : "{level}/10 • {software_complexity}"
*/
pub fn format_complexity(complexity: &ProjectComplexity) -> String {
    format!(
        "{}/10 • {}",
        complexity.level, complexity.software_complexity,
    )
}
