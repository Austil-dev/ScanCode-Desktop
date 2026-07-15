<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { onMount, onDestroy } from "svelte";
  import PageTransition from "$lib/components/PageTransition.svelte";

  interface RecentProject {
    path: string;
    name: string;
    openedAt: number;
  }

  let projectPath = $state("");
  let isLoading = $state(false);
  let errorMessage = $state("");
  let recentProjects = $state<RecentProject[]>([]);
  let windowWidth = $state(typeof window !== 'undefined' ? window.innerWidth : 1024);
  let showTransition = $state(false);
  let isDragging = $state(false);
  let unlistenFileDrop: (() => void) | null = null;

  // Charger les projets récents au montage
  onMount(async () => {
    loadRecentProjects();
    windowWidth = window.innerWidth;
    window.addEventListener('resize', () => {
      windowWidth = window.innerWidth;
    });

    // Écouter les événements de file drop de Tauri
    try {
      const webview = getCurrentWebviewWindow();
      
      unlistenFileDrop = await webview.onDragDropEvent((event) => {
        console.log('File drop event:', event);
        
        if (event.payload.type === 'over') {
          // L'utilisateur survole l'application avec un fichier
          isDragging = true;
          
          // Mettre la fenêtre au premier plan
          webview.setFocus().catch(err => console.error("Erreur focus:", err));
          webview.unminimize().catch(err => console.error("Erreur unminimize:", err));
        } else if (event.payload.type === 'drop') {
          // L'utilisateur a déposé le fichier
          isDragging = false;
          const paths = event.payload.paths;
          
          if (paths && paths.length > 0) {
            const droppedPath = paths[0];
            console.log('Dropped path:', droppedPath);
            openProject(droppedPath);
          }
        } else if (event.payload.type === 'leave') {
          // L'utilisateur a annulé (sorti de la fenêtre)
          isDragging = false;
        }
      });
    } catch (error) {
      console.error('Erreur lors de la configuration du file drop:', error);
    }
  });

  onDestroy(() => {
    // Nettoyer l'écouteur
    if (unlistenFileDrop) {
      unlistenFileDrop();
    }
  });

  /* Charger les projets récents depuis la mémoire local */
  function loadRecentProjects() {
    try {
      const stored = localStorage.getItem("recentProjects");
      if (stored) {
        recentProjects = JSON.parse(stored);
        // Trier par date la plus récente
        recentProjects.sort((a, b) => b.openedAt - a.openedAt);
      }
    } catch (error) {
      console.error("Erreur lors du chargement des projets récents:", error);
    }
  }

  /* Sauvegarder un projet dans la partie "Projets récents" */
  function saveRecentProject(path: string) {
    try {
      const name = path.split(/[\\\/]/).pop() || path;
      const newProject: RecentProject = {
        path,
        name,
        openedAt: Date.now()
      };

      // Vérifier si le projet existe déjà
      const index = recentProjects.findIndex(p => p.path === path);
      if (index !== -1) {
        recentProjects.splice(index, 1);
      }

      // Ajouter au début et limiter à 10 projets
      recentProjects.unshift(newProject);
      if (recentProjects.length > 10) {
        recentProjects = recentProjects.slice(0, 10);
      }

      localStorage.setItem("recentProjects", JSON.stringify(recentProjects));
      recentProjects = recentProjects; // Trigger reactivity
    } catch (error) {
      console.error("Erreur lors de la sauvegarde du projet:", error);
    }
  }

  /* Ouvrir le sélecteur de dossier */
  async function selectFolder() {
    try {
      isLoading = true;
      errorMessage = "";
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Sélectionner un projet"
      });

      if (selected) {
        projectPath = selected as string;
      }
    } catch (error) {
      errorMessage = `Erreur: ${error}`;
    } finally {
      isLoading = false;
    }
  }

  /* Ouvrir le projet sélectionné */
  async function openProject(path?: string) {
    const pathToOpen = path || projectPath;
    
    if (!pathToOpen) {
      errorMessage = "Veuillez sélectionner un dossier";
      return;
    }

    try {
      isLoading = true;
      errorMessage = "";
      
      // Afficher l'écran de transition
      showTransition = true;
      
      // Attendre un peu pour que l'animation se lance
      await new Promise(resolve => setTimeout(resolve, 500));

      // Sauvegarder dans les projets récents
      saveRecentProject(pathToOpen);

      // Appeler la commande Rust pour ouvrir le projet
      await invoke("open_project", { path: pathToOpen });

      // Réinitialiser l'input après ouverture réussie
      projectPath = "";
      
      // Attendre un peu avant de cacher la transition
      await new Promise(resolve => setTimeout(resolve, 1000));
    } catch (error) {
      errorMessage = `Erreur: ${error}`;
      showTransition = false;
    } finally {
      isLoading = false;
    }
  }

  /* Formater et l'heure la date au format "jj/mm/aaaa hh:mm" */
  function formatDate(timestamp: number): string {
    const date = new Date(timestamp);
    return date.toLocaleDateString("fr-FR", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit"
    });
  }

  /* Tronquer le texte long avec des points de suspension basé sur 80vw */
  function truncate(text: string): string {
    const maxWidth = Math.max(windowWidth * 0.8, 300); // 80vw ou min 300px
    const estimatedCharWidth = 8; // pixels par caractère (approximation)
    const maxChars = Math.floor(maxWidth / estimatedCharWidth);
    
    if (text.length > maxChars && maxChars > 3) {
      return text.substring(0, maxChars - 3) + "...";
    }
    return text;
  }
</script>

<main class="dashboard">

    <div class="header">
      <div class="header-toolbar">
        <div class="toolbar-icons">
          <button type="button" class="toolbar-icon-btn" title="Ouvrir un projet" onclick={selectFolder} disabled={isLoading}>
            <span>📁</span>
          </button>
          <button type="button" class="toolbar-icon-btn" title="Paramètres" disabled>
            <span>⚙️</span>
          </button>
          <button type="button" class="toolbar-icon-btn" title="Informations" disabled>
            <span>ℹ️</span>
          </button>
        </div>
        {#if recentProjects.length > 0}
          <button type="button" class="header-action-btn" onclick={() => openProject(recentProjects[0].path)} disabled={isLoading}>
            <span class="action-icon">⚡</span>
            <span class="action-text">Dernier projet</span>
            <span class="arrow-icon">→</span>
          </button>
        {/if}
      </div>
    </div>

  <div 
    class="dashboard-container"
    role="region"
    aria-label="Zone de dépôt de projet"
  >
    {#if errorMessage}
      <div class="error-message">
        {errorMessage}
      </div>
    {/if}

    <div class="project-selector">
      <label class="project-label" for="project-path">Sélectionnez un projet pour continuer :</label>
      <div class="project-input-row">
        <div class="path-input-container">
          <input 
            id="project-path"
            type="text" 
            placeholder="Chemin du projet..." 
            bind:value={projectPath}
            class="path-input"
          />
          <div class="input-actions">
            <button 
              type="button"
              class="input-action-btn folder-btn"
              onclick={selectFolder}
              disabled={isLoading}
              title="Parcourir un dossier"
            >
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path>
              </svg>
            </button>
            {#if projectPath}
              <button 
                type="button"
                class="input-action-btn clear-btn"
                onclick={() => projectPath = ""}
                disabled={isLoading}
                title="Effacer le chemin"
              >
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <line x1="18" y1="6" x2="6" y2="18"></line>
                  <line x1="6" y1="6" x2="18" y2="18"></line>
                </svg>
              </button>
            {/if}
          </div>
        </div>
        <button
          type="button"
          class="continue-btn"
          onclick={() => openProject()}
          disabled={isLoading || !projectPath}
        >
          {isLoading ? "..." : "Continuer"}
        </button>
      </div>
    </div>

    {#if recentProjects.length > 0}
      <div class="recent-projects">
        <h2>Projets récents</h2>
        <div class="projects-list">
          {#if isDragging}
            <div class="project-item drop-zone-item">
              <div class="drop-indicator">
                <div class="plus-icon">+</div>
                <div class="drop-text">Déposer le dossier ici</div>
              </div>
            </div>
          {/if}
          {#each recentProjects as project (project.path)}
            <div class="project-item">
              <button
                type="button"
                class="project-button"
                onclick={() => openProject(project.path)}
                disabled={isLoading}
              >
                <div class="project-info">
                  <div class="project-header">
                    <div class="project-name">{truncate(project.name)}</div>
                    <div class="project-date">{formatDate(project.openedAt)}</div>
                  </div>
                  <div class="project-path" title={project.path}>{truncate(project.path)}</div>
                </div>
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <footer class="app-footer">
    <div class="footer-content">
      <div class="footer-section">
        <span class="footer-label">Version:</span>
        <span class="footer-value">v0.1.0</span>
      </div>
      <div class="footer-separator">|</div>
      <div class="footer-section">
        <span class="footer-label">Stockage:</span>
        <span class="footer-value">Local uniquement</span>
      </div>
      <div class="footer-separator">|</div>
      <div class="footer-section">
        <span class="footer-label">Raccourci:</span>
        <span class="footer-value">Glisser-déposer un projet ici.</span>
      </div>
    </div>
  </footer>

</main>

<PageTransition show={showTransition} />

<style>
  /* Style CSS du titre moderne code/dev */

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
    overflow: hidden;
  }
 
  /* Header fixe en haut */
  .header {
    position: fixed;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 100%;
    z-index: 999;
    display: flex;
    background-color: #112;
    justify-content: center;
    text-align: center;
  } 

  .header-toolbar {
    display: flex;
    gap: 20px;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 12px 20px;
  }

  .toolbar-icons {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .toolbar-icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.9rem;
    line-height: 1;
    padding: 4px 6px;
    transition: all 0.3s ease;
    border-radius: 6px;
    opacity: 0.7;
  }

  .toolbar-icon-btn:hover:not(:disabled) {
    background: rgba(85, 154, 238, 0.15);
    opacity: 1;
    transform: scale(1.08);
  }

  .toolbar-icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .arrow-icon {
    margin-left: 0.3rem;
    font-size: 0.9em;
  }

  .header-toolbar {
    display: flex;
    gap: 20px;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 12px 20px;
  }

  .toolbar-icons {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .toolbar-icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.2rem;
    line-height: 1;
    padding: 6px 8px;
    transition: all 0.3s ease;
    border-radius: 6px;
    opacity: 0.8;
  }

  .toolbar-icon-btn:hover:not(:disabled) {
    background: rgba(85, 154, 238, 0.15);
    opacity: 1;
    transform: scale(1.1);
  }

  .toolbar-icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .arrow-icon {
    margin-left: 0.3rem;
    font-size: 0.9em;
  }

  .header-action-btn {
    background: #559aee;
    border: none;
    border-radius: 8px;
    padding: 0.6rem 1.2rem;
    color: rgba(255, 255, 255, 0.95);
    font-family: 'Fira Code', monospace;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    white-space: nowrap;
  }

  .header-action-btn:hover:not(:disabled) {
    background: #5568d3;
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(85, 154, 238, 0.4);
  }

  .header-action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .dashboard {
    width: 100%;
    height: 100vh;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    position: relative;
    padding: 20px;
    padding-top: 80px;
    padding-bottom: 70px;
    overflow: hidden;
  }

  /* Conteneur principal du tableau de bord */
  .dashboard-container {
    background: rgba(20, 20, 28, 0.8);
    border: 1px solid #2b2b3d;
    border-radius: 12px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.6);
    padding: 20px 35px;
    max-width: 95vw;
    width: 100%;
    height: 100%;
    backdrop-filter: blur(10px);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .error-message {
    padding: 12px 16px;
    background: #3e1f1f;
    color: #ff6b6b;
    border: 1px solid #5c2e2e;
    border-radius: 8px;
    font-size: 0.95rem;
    margin-bottom: 20px;
  }

  .project-selector {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 30px;
    flex-shrink: 0;
  }

  .project-label {
    font-family: 'Fira Code', monospace;
    font-size: 0.9rem;
    color: #8ea6d3;
    font-weight: 600;
    text-align: left;
    margin: 0;
  }

  .project-input-row {
    display: flex;
    gap: 8px;
    align-items: center;
    width: 100%;
  }

  .path-input-container {
    flex: 1;
    position: relative;
    display: flex;
    align-items: center;
  }

  .path-input {
    width: 100%;
    padding: 0.6rem 1rem;
    padding-right: 100px;
    border: 1px solid #3b3b52;
    border-radius: 8px;
    font-family: 'Fira Code', monospace;
    font-size: 0.85rem;
    height: 2.6rem;
    background: rgba(30, 30, 42, 0.8);
    color: #d6d6de;
    transition: all 0.3s ease;
  }

  .path-input:focus {
    outline: none;
    border-color: #559aee;
    box-shadow: 0 0 0 3px rgba(85, 154, 238, 0.2);
  }

  .path-input::placeholder {
    color: #6b7280;
    font-style: italic;
  }

  .input-actions {
    position: absolute;
    right: 6px;
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .input-action-btn {
    background: transparent;
    border: none;
    cursor: pointer;
    padding: 4px 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: #8ea6d3;
    transition: all 0.2s ease;
    border-radius: 4px;
    opacity: 0.8;
  }

  .input-action-btn:hover:not(:disabled) {
    background: rgba(85, 154, 238, 0.15);
    opacity: 1;
  }

  .input-action-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .input-action-btn.clear-btn:hover:not(:disabled) {
    color: #ff6b6b;
  }

  .continue-btn {
    padding: 0.6rem 1.2rem;
    background: #559aee;
    color: rgba(255, 255, 255, 0.95);
    border: none;
    border-radius: 8px;
    font-family: 'Fira Code', monospace;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
    white-space: nowrap;
    height: 2.6rem;
    flex-shrink: 0;
  }

  .continue-btn:hover:not(:disabled) {
    background: #0e0e0e;
    color: rgba(0, 255, 255, 0.95);
    transform: translateY(-2px);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.8),
                0 0 30px rgba(0, 255, 255, 0.6),
                0 0 50px rgba(0, 255, 255, 0.4);
  }

  .continue-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .recent-projects {
    padding-top: 0;
    margin-top: 0; 
    width: 100%;
    display: flex;
    flex-direction: column;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  .recent-projects h2 {
    font-family: 'Fira Code', monospace;
    font-size: 0.85rem;
    color: #8ea6d3;
    margin-bottom: 15px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 1px;
    flex-shrink: 0;
  }

  .projects-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    padding-right: 8px;
  }

  .projects-list::-webkit-scrollbar {
    width: 8px;
  }

  .projects-list::-webkit-scrollbar-track {
    background: rgba(30, 30, 42, 0.4);
    border-radius: 4px;
  }

  .projects-list::-webkit-scrollbar-thumb {
    background: rgba(85, 154, 238, 0.4);
    border-radius: 4px;
    transition: background-color 0.3s ease;
  }

  .projects-list::-webkit-scrollbar-thumb:hover {
    background: rgba(85, 154, 238, 0.7);
  }

  .project-item {
    flex-shrink: 0;
    display: flex;
    gap: 0;
    align-items: stretch;
    width: 100%;
    overflow: hidden;
  }

  .project-button {
    background: rgba(30, 30, 42, 0.6);
    border: 1px solid #3b3b52;
    border-radius: 8px;
    padding: 12px 14px;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
    display: flex;
    align-items: center;
    min-width: 0;
    width: 100%;
  }

  .project-button:hover:not(:disabled) {
    background: rgba(40, 40, 54, 0.8);
    border-color: #559aee;
    box-shadow: 0 4px 12px rgb(0, 115, 255),
                0 1px 3px rgb(0, 238, 255),
                0 1px 2px rgb(0, 162, 255);
    transform: translateY(-1px);
  }

  .project-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .project-info {
    width: 100%;
    max-width: 77vw;
  }

  .project-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    margin-bottom: 6px;
  }

  .project-name {
    font-family: 'Fira Code', monospace;
    font-weight: 600;
    color: #d6d6de;
    font-size: 0.9rem;
    letter-spacing: 0.3px;
    text-align: left;
  }

  .project-date {
    font-family: 'Fira Code', monospace;
    font-size: 0.7rem;
    color: #6b7280;
    opacity: 0.8;
    text-align: right;
  }

  .project-path {
    font-size: 0.75rem;
    color: #7b8794;
    font-family: 'Fira Code', monospace;
    text-align: left;
    width: 100%;
    position: relative;
    display: flex;
    opacity: 0.9;
  }

  .app-footer {
    position: fixed;
    left: 0;
    right: 0;
    bottom: 0;
    width: 100%;
    z-index: 1000;
    background: rgba(15, 15, 15, 0.95);
    border-top: 1px solid #2b2b3d;
    backdrop-filter: blur(10px);
  }

  .footer-content {
    padding: 8px 20px;
    color: #c9c9d1;
    font-size: 0.75rem;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .footer-section {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .footer-label {
    font-weight: 600;
    color: #8ea6d3;
  }

  .footer-separator {
    color: #5b6a86;
    user-select: none;
  }

  .footer-value {
    color: #b5b5c0;
  }

  /* Styles pour le drag and drop */
  .drop-zone-item {
    animation: pulse 1.5s ease-in-out infinite;
  }

  .drop-indicator {
    width: 100%;
    min-height: 80px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.2) 0%, rgba(118, 75, 162, 0.2) 100%);
    border: 2px dashed #667eea;
    border-radius: 12px;
    padding: 20px;
    transition: all 0.3s ease;
  }

  .drop-indicator:hover {
    background: linear-gradient(135deg, rgba(102, 126, 234, 0.3) 0%, rgba(118, 75, 162, 0.3) 100%);
    border-color: #7d8ff5;
  }

  .plus-icon {
    font-size: 3rem;
    color: #667eea;
    font-weight: 300;
    line-height: 1;
    margin-bottom: 10px;
  }

  .drop-text {
    color: #667eea;
    font-size: 0.95rem;
    font-weight: 500;
  }

  @keyframes pulse {
    0%, 100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.02);
      opacity: 0.9;
    }
  }
</style>
