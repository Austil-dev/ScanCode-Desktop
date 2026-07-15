<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";

  // Définition de l'interface pour représenter un nœud de l'arborescence du projet
  interface TreeNode {
    name: string; // Nom du fichier ou du dossier
    path: string; // Chemin complet du nœud
    is_dir: boolean; // Indique si le nœud est un dossier ou un fichier
    size: number; // Ajout de la taille en octets
    children: TreeNode[]; // Enfants du nœud, vide pour les fichiers
  }

  // Variables d'état pour stocker le chemin, la structure et les noeuds du projet
  let projectPath = $state("");
  let treeData = $state<TreeNode | null>(null);
  let expandedNodes = $state<Set<string>>(new Set());

  // Effet pour charger la structure du projet lorsque le composant est monté ou lorsque le chemin du projet change
  $effect(() => {
    const params = new URLSearchParams(window.location.search); // Paramètres de l'URL
    projectPath = params.get("path") || ""; // Chemin du projet à partir des paramètres de l'URL
    
    // Charger la structure du projet si un chemin est fourni
    if (projectPath) {
      loadTreeStructure(); // Charger la structure du projet depuis le backend
    }
  });

  // Fonction pour charger la structure du projet depuis le backend
  async function loadTreeStructure() {
    // Si aucun chemin de projet n'est fourni, réinitialiser les données de l'arborescence
    if (!projectPath) {
      treeData = null;
      return; 
    }

    try {
      // Appeler la fonction Rust pour obtenir la structure du projet depuis le backend
      treeData = await invoke<TreeNode>("get_project_structure", { path: projectPath });
      
      // Étendre automatiquement le premier niveau
      if (treeData && treeData.children) {
        // Afficher le projet lui-même et ses enfants immédiats
        expandedNodes = new Set([treeData.path]);
      }
    } catch (error) {
      // Erreur si la structure ne peut être chargée ou que le projet n'est pas trouvé
      console.error("Erreur lors du chargement de la structure:", error);
      treeData = null;
    }
  }

  // Fonction pour basculer l'état d'expansion d'un nœud
  function toggleNode(path: string) {
    if (expandedNodes.has(path)) {
      expandedNodes.delete(path);
    } else {
      expandedNodes.add(path);
    }
    expandedNodes = new Set(expandedNodes);
  }

  // Formate une taille en octets en une chaîne lisible (ex: 1.5 MB)
  function formatSize(bytes: number): string {
    if (!bytes || bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  // Retourne une icône emoji basée sur l'extension du fichier ou le type de dossier
  function getFileIcon(node: TreeNode): string {
    if (node.is_dir) return "📁";
    
    const ext = node.name.split('.').pop()?.toLowerCase() || "";
    const iconMap: Record<string, string> = {
      'js': '📜', 'ts': '📘', 'tsx': '⚛️', 'jsx': '⚛️',
      'html': '🌐', 'css': '🎨', 'scss': '🎨', 'sass': '🎨',
      'json': '📋', 'xml': '📋', 'yaml': '📋', 'yml': '📋',
      'md': '📝', 'txt': '📄',
      'png': '🖼️', 'jpg': '🖼️', 'jpeg': '🖼️', 'gif': '🖼️', 'svg': '🖼️',
      'mp4': '🎬', 'avi': '🎬', 'mov': '🎬',
      'mp3': '🎵', 'wav': '🎵',
      'zip': '🗜️', 'rar': '🗜️', '7z': '🗜️',
      'pdf': '📕',
      'rs': '🦀', 'go': '🐹', 'py': '🐍', 'java': '☕',
      'c': '©️', 'cpp': '©️', 'h': '©️',
      'exe': '⚙️', 'dll': '⚙️', 'so': '⚙️'
    };
    
    return iconMap[ext] || "📄"; // Icône par défaut pour les fichiers inconnus
  }
</script>

<!-- Composant récursif pour afficher les nœuds de l'arborescence -->
{#snippet TreeNode({ node, level, expandedNodes, toggleNode, getFileIcon, formatSize }: { node: TreeNode, level: number, expandedNodes: Set<string>, toggleNode: (path: string) => void, getFileIcon: (node: TreeNode) => string, formatSize: (bytes: number) => string })}
  {@const isExpanded = expandedNodes.has(node.path)}
  {@const hasChildren = node.is_dir && node.children && node.children.length > 0}
  
  <div class="tree-node" style="padding-left: {level * 20}px">
    <!-- Contenu du nœud avec gestion de l'expansion -->
    <div 
      class="node-content" 
      onclick={() => hasChildren && toggleNode(node.path)}
      role="button"
      tabindex="0"
      onkeydown={(e) => (e.key === 'Enter' || e.key === ' ') && hasChildren && toggleNode(node.path)}
    >
      <div class="node-left">
        {#if hasChildren}
        <!-- Icône d'expansion -->
          <span class="expand-icon">{isExpanded ? '▼' : '▶'}</span>
        {:else}
        <!-- Placeholder pour aligner les icônes des fichiers sans enfants -->
          <span class="expand-icon-placeholder"></span>
        {/if}
        <!-- Icône du fichier ou du dossier -->
        <span class="node-icon">{getFileIcon(node)}</span>
        <!-- Nom du fichier ou du dossier -->
        <span class="node-name">{node.name}</span>
      </div>
      <!-- Affichage de la taille du fichier ou du dossier -->
      <span class="node-size">{node.size ? formatSize(node.size) : "0 B"}</span>
    </div>
    <!-- Enfants du nœud -->
    {#if isExpanded && hasChildren}
      {#each node.children as child}
        <!-- Rendu récursif des enfants -->
        {@render TreeNode({ node: child, level: level + 1, expandedNodes, toggleNode, getFileIcon, formatSize })}
      {/each}
    {/if}
  </div>
{/snippet}

<main class="page-container">
  <!-- En-tête de la page -->
  <div class="header" transition:fade={{ duration: 300 }}>
    <h1>Structure</h1>
  </div>

  <!-- Contenu principal de la page -->
  <div class="content-wrapper" transition:fade={{ duration: 300, delay: 100 }}>
    <!-- Affichage d'un message d'erreur si aucun projet n'est sélectionné -->
    {#if !treeData}
      <div class="error-message">Aucun projet sélectionné</div>
    {:else}
    <!-- Affichage de la structure du projet -->
      <div class="tree-container">
        <!-- Conteneur de l'arborescence -->
        <div class="tree-header">
          <!-- En-tête de l'arborescence -->
          <div>
            <!-- Nom du projet et chemin d'accès -->
            <h2>{treeData.name}</h2>
            <!-- Affichage du chemin complet du projet -->
            <p class="tree-path">{treeData.path}</p>
          </div>
          <!-- Affichage de la taille totale du projet -->
          <div class="total-size">
            <!-- Label pour la taille totale -->
            <span class="size-label">Taille totale:</span>
            <!-- Valeur de la taille totale -->
            <span class="size-value">{treeData.size ? formatSize(treeData.size) : "Calcul en cours..."}</span>
          </div>
        </div>

        <!-- Contenu de l'arborescence -->
        <div class="tree-content">.
          <!-- Vérification de la présence d'enfants dans la structure du projet -->
          {#if treeData.children && treeData.children.length > 0}
          <!-- Rendu récursif des enfants -->
            {#each treeData.children as child}
              <!-- Rendu récursif d'un enfant -->
              {@render TreeNode({ node: child, level: 0, expandedNodes, toggleNode, getFileIcon, formatSize })}
            {/each}
          <!-- Affichage d'un message si aucun fichier n'est trouvé dans le projet -->
          {:else}
            <!-- Message indiquant qu'aucun fichier n'a été trouvé -->
            <div class="empty-message">Aucun fichier trouvé</div>
          {/if}
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  /* */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  /* Styles globaux pour la page */
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;
    color: #e0e0e0;
    background: linear-gradient(140deg, rgba(100, 17, 255, 0.8) 0%, rgb(40, 126, 255, 0.8) 100%);
    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  /* Conteneur principal de la page */
  .page-container {
    width: 100%;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: transparent;
  }

  /* En-tête fixe en haut de la page */
  .header {
    position: fixed;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    z-index: 999;
    display: flex;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.95) 0%, rgba(33, 33, 66, 0.95) 100%);
    backdrop-filter: blur(10px);
    justify-content: center;
    text-align: center;
    padding: 20px 30px;
    border-bottom: 2px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
  }

  /* Style de l'en-tête de la page */
  .header h1 {
    color: #667eea;
    font-size: 2rem;
    margin: 0;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-shadow: 0 2px 10px rgba(103, 126, 234, 0.5);
  }

  /* Conteneur pour le contenu principal de la page */
  .content-wrapper {
    flex: 1;
    padding: 100px 30px 30px;
    overflow-y: auto;
    overflow-x: hidden;
    scroll-behavior: smooth;
  }

  /* Conteneur de l'arborescence */
  .tree-container {
    max-width: 1400px;
    margin: 0 auto;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    border-radius: 16px;
    border: 1px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    overflow: hidden;
  }

  /* En-tête de l'arborescence avec le nom du projet et la taille totale */
  .tree-header {
    padding: 25px 30px;
    background: linear-gradient(135deg, rgba(103, 126, 234, 0.2) 0%, rgba(103, 126, 234, 0.1) 100%);
    border-bottom: 1px solid rgba(103, 126, 234, 0.3);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 20px;
  }

  .tree-header h2 {
    color: #667eea;
    font-size: 1.5rem;
    margin-bottom: 8px;
    font-weight: 600;
  }

  .tree-path {
    color: #a0a0a0;
    font-family: monospace;
    font-size: 0.9rem;
    word-break: break-all;
  }

  .total-size {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
    padding: 10px 15px;
    background: rgba(103, 126, 234, 0.15);
    border-radius: 8px;
    border: 1px solid rgba(103, 126, 234, 0.3);
  }

  .size-label {
    font-size: 0.85rem;
    color: #a0a0a0;
  }

  .size-value {
    font-size: 1.1rem;
    color: #667eea;
    font-weight: 600;
  }

  .tree-content {
    padding: 20px;
    max-height: calc(100vh - 300px);
    overflow-y: auto;
  }

  .tree-node {
    margin: 2px 0;
  }

  .node-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .node-content:hover {
    background: rgba(103, 126, 234, 0.15);
  }

  .node-left {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
    min-width: 0;
  }

  .expand-icon {
    width: 16px;
    font-size: 0.8rem;
    color: #667eea;
    flex-shrink: 0;
  }

  .expand-icon-placeholder {
    width: 16px;
    flex-shrink: 0;
  }

  .node-icon {
    font-size: 1.2rem;
    flex-shrink: 0;
  }

  .node-name {
    flex: 1;
    color: #e0e0e0;
    font-size: 0.95rem;
    word-break: break-word;
  }

  .node-size {
    color: #667eea;
    font-size: 0.85rem;
    font-weight: 500;
    white-space: nowrap;
    padding: 4px 8px;
    background: rgba(103, 126, 234, 0.1);
    border-radius: 6px;
    flex-shrink: 0;
  }

  .error-message,
  .empty-message {
    background: linear-gradient(135deg, rgba(62, 31, 31, 0.9) 0%, rgba(92, 46, 46, 0.9) 100%);
    color: #ff6b6b;
    border: 1px solid rgba(255, 107, 107, 0.3);
    box-shadow: 0 8px 32px rgba(255, 107, 107, 0.2);
    padding: 20px;
    border-radius: 12px;
    text-align: center;
    margin: 20px;
  }

  .empty-message {
    background: linear-gradient(135deg, rgba(40, 40, 60, 0.9) 0%, rgba(50, 50, 70, 0.9) 100%);
    color: #a0a0a0;
    border-color: rgba(160, 160, 160, 0.3);
  }

  @media (max-width: 768px) {
    .header h1 {
      font-size: 1.5rem;
    }

    .content-wrapper {
      padding: 80px 15px 15px;
    }

    .tree-header {
      padding: 20px;
      flex-direction: column;
      align-items: flex-start;
    }

    .total-size {
      align-self: stretch;
    }

    .tree-header h2 {
      font-size: 1.3rem;
    }

    .tree-content {
      padding: 15px;
    }

    .node-content {
      padding: 6px 10px;
      gap: 6px;
    }

    .node-size {
      font-size: 0.75rem;
      padding: 3px 6px;
    }
  }

  @media (max-width: 480px) {
    .content-wrapper {
      padding: 70px 10px 10px;
    }

    .tree-header {
      padding: 15px;
    }

    .tree-header h2 {
      font-size: 1.2rem;
    }

    .tree-path {
      font-size: 0.8rem;
    }

    .tree-content {
      padding: 10px;
    }

    .node-content {
      padding: 5px 8px;
      gap: 5px;
    }

    .node-name {
      font-size: 0.9rem;
    }

    .node-size {
      font-size: 0.7rem;
      padding: 2px 5px;
    }
  }
</style>