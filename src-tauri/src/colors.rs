use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Note : Serialize et Deserialize sont des traits de la bibliothèque Serde qui permettent de convertir des structures Rust
//en formats de données sérialisés (comme JSON) et vice versa.
#[derive(Serialize, Deserialize, Clone)]
/*
    Une structure représentant une palette de couleurs.
*/
pub struct ColorPalette {
    pub bg: String,
    pub border: String,
}

/*
    Retourne la palette de couleurs complète pour les catégories et langages.
*/
pub fn get_color_palette() -> HashMap<String, ColorPalette> {
    // HashMap est une structure de données qui stocke des paires clé-valeur (clé = catégorie/langage, valeur = palette de couleurs).
    let mut palette = HashMap::new();

    // Langages de programmation et couleurs associées
    palette.insert(
        "TypeScript".to_string(),
        ColorPalette {
            bg: "rgba(0, 122, 204, 0.8)".to_string(),
            border: "rgba(0, 122, 204, 1)".to_string(),
        },
    );
    palette.insert(
        "JavaScript".to_string(),
        ColorPalette {
            bg: "rgba(240, 219, 79, 0.8)".to_string(),
            border: "rgba(240, 219, 79, 1)".to_string(),
        },
    );
    palette.insert(
        "Svelte".to_string(),
        ColorPalette {
            bg: "rgba(255, 62, 0, 0.8)".to_string(),
            border: "rgba(255, 62, 0, 1)".to_string(),
        },
    );
    palette.insert(
        "Rust".to_string(),
        ColorPalette {
            bg: "rgba(222, 165, 132, 0.8)".to_string(),
            border: "rgba(222, 165, 132, 1)".to_string(),
        },
    );
    palette.insert(
        "Python".to_string(),
        ColorPalette {
            bg: "rgba(53, 114, 165, 0.8)".to_string(),
            border: "rgba(53, 114, 165, 1)".to_string(),
        },
    );
    palette.insert(
        "Java".to_string(),
        ColorPalette {
            bg: "rgba(176, 114, 25, 0.8)".to_string(),
            border: "rgba(176, 114, 25, 1)".to_string(),
        },
    );
    palette.insert(
        "C".to_string(),
        ColorPalette {
            bg: "rgba(26, 117, 38, 0.8)".to_string(),
            border: "rgb(26, 117, 26)".to_string(),
        },
    );
    palette.insert(
        "C++".to_string(),
        ColorPalette {
            bg: "rgba(0, 89, 157, 0.8)".to_string(),
            border: "rgba(0, 89, 157, 1)".to_string(),
        },
    );
    palette.insert(
        "C/C++".to_string(),
        ColorPalette {
            bg: "rgba(26, 106, 117, 0.8)".to_string(),
            border: "rgb(26, 88, 117)".to_string(),
        },
    );
    palette.insert(
        "C#".to_string(),
        ColorPalette {
            bg: "rgba(104, 33, 122, 0.8)".to_string(),
            border: "rgba(104, 33, 122, 1)".to_string(),
        },
    );
    palette.insert(
        "Go".to_string(),
        ColorPalette {
            bg: "rgba(0, 173, 216, 0.8)".to_string(),
            border: "rgba(0, 173, 216, 1)".to_string(),
        },
    );
    palette.insert(
        "PHP".to_string(),
        ColorPalette {
            bg: "rgba(119, 123, 180, 0.8)".to_string(),
            border: "rgba(119, 123, 180, 1)".to_string(),
        },
    );
    palette.insert(
        "Ruby".to_string(),
        ColorPalette {
            bg: "rgba(204, 52, 45, 0.8)".to_string(),
            border: "rgba(204, 52, 45, 1)".to_string(),
        },
    );
    palette.insert(
        "Swift".to_string(),
        ColorPalette {
            bg: "rgba(250, 91, 56, 0.8)".to_string(),
            border: "rgba(250, 91, 56, 1)".to_string(),
        },
    );
    palette.insert(
        "Kotlin".to_string(),
        ColorPalette {
            bg: "rgba(124, 123, 255, 0.8)".to_string(),
            border: "rgba(124, 123, 255, 1)".to_string(),
        },
    );

    // Technologies web et fichiers
    palette.insert(
        "CSS".to_string(),
        ColorPalette {
            bg: "rgba(86, 61, 124, 0.8)".to_string(),
            border: "rgba(86, 61, 124, 1)".to_string(),
        },
    );
    palette.insert(
        "HTML".to_string(),
        ColorPalette {
            bg: "rgba(227, 76, 38, 0.8)".to_string(),
            border: "rgba(227, 76, 38, 1)".to_string(),
        },
    );
    palette.insert(
        "JSON".to_string(),
        ColorPalette {
            bg: "rgba(41, 128, 185, 0.8)".to_string(),
            border: "rgba(41, 128, 185, 1)".to_string(),
        },
    );
    palette.insert(
        "YAML".to_string(),
        ColorPalette {
            bg: "rgba(203, 67, 53, 0.8)".to_string(),
            border: "rgba(203, 67, 53, 1)".to_string(),
        },
    );
    palette.insert(
        "XML".to_string(),
        ColorPalette {
            bg: "rgba(149, 165, 166, 0.8)".to_string(),
            border: "rgba(149, 165, 166, 1)".to_string(),
        },
    );
    palette.insert(
        "Markdown".to_string(),
        ColorPalette {
            bg: "rgba(52, 73, 94, 0.8)".to_string(),
            border: "rgba(52, 73, 94, 1)".to_string(),
        },
    );
    palette.insert(
        "SQL".to_string(),
        ColorPalette {
            bg: "rgba(231, 76, 60, 0.8)".to_string(),
            border: "rgba(231, 76, 60, 1)".to_string(),
        },
    );
    palette.insert(
        "Shell".to_string(),
        ColorPalette {
            bg: "rgba(137, 224, 81, 0.8)".to_string(),
            border: "rgba(137, 224, 81, 1)".to_string(),
        },
    );

    // Catégories de fichiers
    palette.insert(
        "Config".to_string(),
        ColorPalette {
            bg: "rgba(41, 128, 185, 0.8)".to_string(),
            border: "rgba(41, 128, 185, 1)".to_string(),
        },
    );
    palette.insert(
        "Exec".to_string(),
        ColorPalette {
            bg: "rgba(231, 76, 60, 0.8)".to_string(),
            border: "rgba(231, 76, 60, 1)".to_string(),
        },
    );
    palette.insert(
        "Images".to_string(),
        ColorPalette {
            bg: "rgba(155, 89, 182, 0.8)".to_string(),
            border: "rgba(155, 89, 182, 1)".to_string(),
        },
    );
    palette.insert(
        "Audio".to_string(),
        ColorPalette {
            bg: "rgba(46, 204, 113, 0.8)".to_string(),
            border: "rgba(46, 204, 113, 1)".to_string(),
        },
    );
    palette.insert(
        "Video".to_string(),
        ColorPalette {
            bg: "rgba(230, 126, 34, 0.8)".to_string(),
            border: "rgba(230, 126, 34, 1)".to_string(),
        },
    );
    palette.insert(
        "Docs".to_string(),
        ColorPalette {
            bg: "rgba(52, 152, 219, 0.8)".to_string(),
            border: "rgba(52, 152, 219, 1)".to_string(),
        },
    );
    palette.insert(
        "Scripts".to_string(),
        ColorPalette {
            bg: "rgba(137, 224, 81, 0.8)".to_string(),
            border: "rgba(137, 224, 81, 1)".to_string(),
        },
    );
    palette.insert(
        "Autres".to_string(),
        ColorPalette {
            bg: "rgba(149, 165, 166, 0.8)".to_string(),
            border: "rgba(149, 165, 166, 1)".to_string(),
        },
    );

    // Couleur par défaut
    palette.insert(
        "default".to_string(),
        ColorPalette {
            bg: "rgba(149, 165, 166, 0.8)".to_string(),
            border: "rgba(149, 165, 166, 1)".to_string(),
        },
    );

    palette // Retourne la palette de couleurs complète
}
