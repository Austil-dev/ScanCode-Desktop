<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";

  interface ProjectInfo {
    path: string;
    size: number;
    complexity: string;
    code_files_count: number;
    resource_files_count: number;
    executable_count: number;
    total_files_count: number;
    max_depth: number;
    detected_editor: string;
  }

  interface ProjectComplexity {
    level: string;
    notation: string;
    total_cyclomatic: number;
    avg_cyclomatic: number;
    total_npath: number;
    total_loc: number;
    avg_maintainability: number;
    files_analyzed: number;
  }

  let projectPath = $state("");
  let projectInfo = $state<ProjectInfo | null>(null);
  let complexityDetails = $state<ProjectComplexity | null>(null);
  let showComplexityDetails = $state(false);

  $effect(() => {
    const params = new URLSearchParams(window.location.search);
    projectPath = params.get("path") || "";
    
    if (projectPath) {
      loadProjectInfo();
      loadComplexityDetails();
    }
  });

  async function loadProjectInfo() {
    if (!projectPath) {
      projectInfo = null;
      return;
    }

    try {
      projectInfo = await invoke<ProjectInfo>("get_project_info", { path: projectPath });
    } catch (error) {
      console.error("Erreur lors du chargement des informations:", error);
      projectInfo = null;
    }
  }

  async function loadComplexityDetails() {
    if (!projectPath) {
      complexityDetails = null;
      return;
    }

    try {
      complexityDetails = await invoke<ProjectComplexity>("get_project_complexity", { path: projectPath });
    } catch (error) {
      console.error("Erreur lors du chargement de la complexité:", error);
      complexityDetails = null;
    }
  }

  // Convertit la taille d'un élément en octets (nombre) en une chaîne lisible.
  function stringifySize(size: number): string {
    if (size < 1024) return `${size} octets`;

    const units = ["Ko", "Mo", "Go", "To"];
    let index = -1;
    let s = size;
    do {
      s /= 1024;
      index++;
    } while (s >= 1024 && index < units.length - 1);

    return `${s.toFixed(2)} ${units[index]}`;
  }

  // Retourne une couleur hexadécimale en fonction de la complexité.
  function getComplexityColor(complexity: string): string {
    const level = complexity.split(" • ")[0];
    switch (level) {
      case "Acceptable": return "#4ade80";
      case "Moyenne": return "#fbbf24";
      case "Élevée": return "#fb923c";
      case "Très élevée": return "#ef4444";
      // Anciens niveaux pour compatibilité
      case "Simple": return "#4ade80";
      case "Modérée": return "#fbbf24";
      case "Complexe": return "#fb923c";
      case "Très complexe": return "#ef4444";
      default: return "#667eea";
    }
  }
</script>

<main class="page-container">
    <div class="header" transition:fade={{ duration: 300, delay: 100 }}>
      <h1>Informations</h1>
    </div>

    <div class="content-wrapper" transition:fade={{ duration: 300, delay: 200 }}>
      {#if !projectInfo}
        <div class="error-message">Aucun projet sélectionné</div>
      {:else}
        <div class="content">
          <!-- En-tête avec chemin du projet -->
          <div class="path-card">
            <div class="icon-large">📁</div>
            <div class="path-info">
              <h3>Chemin du projet</h3>
              <p class="path-text">{projectInfo.path}</p>
            </div>
          </div>

          <!-- Grille d'informations -->
          <div class="info-grid">
            <!-- Taille du projet -->
            <div class="info-card">
              <div class="card-icon">💾</div>
              <div class="card-content">
                <h3>Taille du projet</h3>
                <p class="card-value">{stringifySize(projectInfo.size)}</p>
              </div>
            </div>

            <!-- Complexité -->
            <div 
              class="info-card clickable" 
              role="button" 
              tabindex="0"
              onclick={() => showComplexityDetails = !showComplexityDetails}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') showComplexityDetails = !showComplexityDetails }}
            >
              <div class="card-icon">⚡</div>
              <div class="card-content">
                <h3>Complexité {showComplexityDetails ? '▼' : '▶'}</h3>
                <p class="card-value" style="color: {getComplexityColor(projectInfo.complexity)}">{projectInfo.complexity}</p>
              </div>
            </div>

            <!-- Profondeur -->
            <div class="info-card">
              <div class="card-icon">📏</div>
              <div class="card-content">
                <h3>Profondeur max</h3>
                <p class="card-value">{projectInfo.max_depth} niveau{projectInfo.max_depth > 1 ? 'x' : ''}</p>
              </div>
            </div>

            <!-- Éditeur détecté -->
            <div class="info-card">
              <div class="card-icon">🛠️</div>
              <div class="card-content">
                <h3>Éditeur détecté</h3>
                <p class="card-value editor">{projectInfo.detected_editor}</p>
              </div>
            </div>

            <!-- Total fichiers -->
            <div class="info-card">
              <div class="card-icon">📊</div>
              <div class="card-content">
                <h3>Total fichiers</h3>
                <p class="card-value">{projectInfo.total_files_count}</p>
              </div>
            </div>

            <!-- Fichiers de code -->
            <div class="info-card">
              <div class="card-icon">📝</div>
              <div class="card-content">
                <h3>Fichiers de code</h3>
                <p class="card-value">{projectInfo.code_files_count}</p>
              </div>
            </div>

            <!-- Fichiers ressources -->
            <div class="info-card">
              <div class="card-icon">🖼️</div>
              <div class="card-content">
                <h3>Fichiers ressources</h3>
                <p class="card-value">{projectInfo.resource_files_count}</p>
              </div>
            </div>

            <!-- Exécutables -->
            <div class="info-card">
              <div class="card-icon">⚙️</div>
              <div class="card-content">
                <h3>Exécutables</h3>
                <p class="card-value">{projectInfo.executable_count}</p>
              </div>
            </div>
    
          </div> <!-- Fin de la grille d'informations -->

          <!-- Détails de complexité (dépliable) -->
          {#if showComplexityDetails && complexityDetails}
            <div class="complexity-details" transition:fade={{ duration: 200 }}>
              <h2>📊 Analyse détaillée de la complexité</h2>
              <div class="details-grid">
                <div class="detail-item">
                  <span class="detail-label">Complexité cyclomatique totale:</span>
                  <span class="detail-value">{complexityDetails.total_cyclomatic}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Complexité cyclomatique moyenne:</span>
                  <span class="detail-value">{complexityDetails.avg_cyclomatic.toFixed(2)}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Complexité NPath totale:</span>
                  <span class="detail-value">{complexityDetails.total_npath.toLocaleString()}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Total lignes de code (LOC):</span>
                  <span class="detail-value">{complexityDetails.total_loc.toLocaleString()}</span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Index de maintenabilité moyen:</span>
                  <span class="detail-value" style="color: {complexityDetails.avg_maintainability >= 70 ? '#4ade80' : complexityDetails.avg_maintainability >= 50 ? '#fbbf24' : '#ef4444'}">
                    {complexityDetails.avg_maintainability.toFixed(1)}%
                  </span>
                </div>
                <div class="detail-item">
                  <span class="detail-label">Fichiers analysés:</span>
                  <span class="detail-value">{complexityDetails.files_analyzed}</span>
                </div>
              </div>
              <div class="complexity-legend">
                <p><strong>Interprétation:</strong></p>
                <ul>
                  <li>Complexité cyclomatique ≤ 10: Code facile à maintenir</li>
                  <li>Complexité NPath ≤ 200: Nombre de tests recommandé</li>
                  <li>Index de maintenabilité ≥ 70: Excellent • 50-69: Bon • &lt;50: Améliorations nécessaires</li>
                </ul>
              </div>
            </div>
          {/if}
        </div> <!-- Fin du contenu -->
      {/if}
    </div>
</main>

<style>
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

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

  .page-container {
    width: 100%;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: transparent;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  /* Header avec le même style que statistiques */
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

  .header h1 {
    color: #667eea;
    font-size: 2rem;
    margin: 0;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-shadow: 0 2px 10px rgba(103, 126, 234, 0.5);
  }

  .content-wrapper {
    flex: 1;
    padding: 100px 30px 30px;
    overflow-y: auto;
    overflow-x: hidden;
    scroll-behavior: smooth;
  }

  .content {
    max-width: 1400px;
    margin: 0 auto;
  }

  .error-message {
    background: linear-gradient(135deg, rgba(62, 31, 31, 0.9) 0%, rgba(92, 46, 46, 0.9) 100%);
    color: #ff6b6b;
    border: 1px solid rgba(255, 107, 107, 0.3);
    box-shadow: 0 8px 32px rgba(255, 107, 107, 0.2);
  }

  .path-card {
    display: flex;
    align-items: center;
    gap: 15px;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    padding: 15px 20px;
    border-radius: 12px;
    border: 1px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    margin-bottom: 20px;
    transition: all 0.3s ease;
  }

  .path-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 12px 40px rgba(103, 126, 234, 0.4);
    border-color: rgba(103, 126, 234, 0.5);
  }

  .icon-large {
    font-size: 2rem;
    flex-shrink: 0;
  }

  .path-info {
    flex: 1;
    min-width: 0;
  }

  .path-info h3 {
    color: #667eea;
    font-size: 0.95rem;
    margin-bottom: 6px;
    font-weight: 600;
  }

  .path-text {
    color: #e0e0e0;
    font-family: monospace;
    font-size: 0.85rem;
    word-break: break-all;
    line-height: 1.4;
  }

  .info-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 15px;
  }

  .info-card {
    display: flex;
    align-items: center;
    gap: 15px;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    padding: 15px;
    border-radius: 12px;
    border: 1px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
  }

  .info-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 40px rgba(103, 126, 234, 0.4);
    border-color: rgba(103, 126, 234, 0.5);
  }

  .card-icon {
    font-size: 1.8rem;
    flex-shrink: 0;
  }

  .card-content {
    flex: 1;
    min-width: 0;
  }

  .card-content h3 {
    color: #667eea;
    font-size: 0.85rem;
    margin-bottom: 5px;
    font-weight: 600;
  }

  .card-value {
    color: #e0e0e0;
    font-size: 1.3rem;
    font-weight: 700;
    line-height: 1.2;
    word-break: break-word;
  }

  .card-value.editor {
    font-size: 1.1rem;
  }

  .clickable {
    cursor: pointer;
  }

  .clickable:hover {
    background: linear-gradient(135deg, rgba(25, 25, 50, 0.95) 0%, rgba(43, 43, 76, 0.95) 100%);
  }

  .complexity-details {
    margin-top: 20px;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    padding: 20px;
    border-radius: 12px;
    border: 1px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  }

  .complexity-details h2 {
    color: #667eea;
    font-size: 1.2rem;
    margin-bottom: 15px;
    font-weight: 600;
  }

  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 12px;
    margin-bottom: 15px;
  }

  .detail-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    background: rgba(103, 126, 234, 0.1);
    border-radius: 8px;
    border: 1px solid rgba(103, 126, 234, 0.2);
  }

  .detail-label {
    color: #a0a0a0;
    font-size: 0.85rem;
    font-weight: 500;
  }

  .detail-value {
    color: #e0e0e0;
    font-size: 1rem;
    font-weight: 700;
  }

  .complexity-legend {
    padding: 12px;
    background: rgba(103, 126, 234, 0.05);
    border-radius: 8px;
    border-left: 3px solid #667eea;
  }

  .complexity-legend p {
    color: #667eea;
    font-size: 0.9rem;
    font-weight: 600;
    margin-bottom: 8px;
  }

  .complexity-legend ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .complexity-legend li {
    color: #a0a0a0;
    font-size: 0.8rem;
    padding: 4px 0;
    padding-left: 15px;
    position: relative;
  }

  .complexity-legend li::before {
    content: "•";
    position: absolute;
    left: 0;
    color: #667eea;
  }

  @media (max-width: 768px) {
    .header h1 {
      font-size: 1.5rem;
    }

    .content-wrapper {
      padding: 80px 15px 15px;
    }

    .path-card {
      flex-direction: column;
      align-items: flex-start;
      padding: 20px;
    }

    .icon-large {
      font-size: 2.5rem;
    }

    .info-grid {
      grid-template-columns: 1fr;
      gap: 15px;
    }

    .info-card {
      padding: 20px;
    }

    .card-icon {
      font-size: 2rem;
    }

    .card-value {
      font-size: 1.5rem;
    }

    .card-value.editor {
      font-size: 1.2rem;
    }
  }

  @media (max-width: 480px) {
    .content-wrapper {
      padding: 70px 10px 10px;
    }

    .path-card {
      padding: 15px;
    }

    .path-info h3 {
      font-size: 1rem;
    }

    .path-text {
      font-size: 0.85rem;
    }

    .info-card {
      padding: 15px;
      gap: 15px;
    }

    .card-icon {
      font-size: 1.8rem;
    }

    .card-content h3 {
      font-size: 0.9rem;
    }

    .card-value {
      font-size: 1.3rem;
    }

    .card-value.editor {
      font-size: 1.1rem;
    }
  }
</style>
