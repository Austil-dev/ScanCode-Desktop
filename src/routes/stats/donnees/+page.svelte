<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { fade } from "svelte/transition";
  import { onMount } from "svelte";

  // Interfaces TypeScript pour les données Git et l'historique local
  interface GitCommit {
    hash: string;
    author: string;
    date: string;
    message: string;
    files_changed: string[];
    insertions: number;
    deletions: number;
  }

  // Interface pour l'historique Git d'un projet
  interface GitHistory {
    commits: GitCommit[];
    total_commits: number;
    total_files_changed: number;
    total_insertions: number;
    total_deletions: number;
  }

  // Interface pour les statistiques globales d'un projet Git
  interface GitStats {
    total_commits: number;
    total_contributors: number;
    first_commit_date: string;
    last_commit_date: string;
    total_branches: number;
  }

  // Interface pour un événement de l'historique local d'un projet
  interface LocalHistoryEvent {
    timestamp: string;
    event_type: string;
    description: string;
    files_affected: string[];
  }

  // Interface pour l'historique local d'un projet
  interface LocalHistory {
    events: LocalHistoryEvent[];
    total_events: number;
    last_scan_date: string | null;
    total_scans: number;
  }

  // États locaux du composant pour stocker les données et l'état de l'interface
  let projectPath = $state("");
  let gitHistory: GitHistory | null = $state(null);
  let gitStats: GitStats | null = $state(null);
  let localHistory: LocalHistory | null = $state(null);
  let uncommittedFiles: string[] = $state([]);
  let recentChanges: string[] = $state([]);
  let hasGit = $state(false);
  let loading = $state(true);
  let error = $state("");
  let commitLimit = $state(50);
  let expandedCommits = $state(new Set<string>());
  let expandedLocalEvents = $state(new Set<string>());

  // Effet pour charger les données à partir des paramètres de l'URL (initialisation du projet)
  $effect(() => {
    const params = new URLSearchParams(window.location.search);
    projectPath = params.get("path") || "";

    if (projectPath) {
      loadHistoryData();
    }
  });

  // Fonction pour charger les données d'historique Git et local du projet
  async function loadHistoryData() {
    if (!projectPath) return;

    loading = true;
    error = "";

    try {
      // Vérifier si c'est un dépôt Git
      hasGit = await invoke<boolean>("check_is_git_repository", {
        path: projectPath,
      });

      // Charger l'historique Git si disponible
      if (hasGit) {
        try {
          // Charger l'historique Git avec une limite de commits depuis le backend
          gitHistory = await invoke<GitHistory>("get_git_project_history", {
            path: projectPath,
            limit: commitLimit,
          });

          // Charger les statistiques Git depuis le backend
          gitStats = await invoke<GitStats>("get_git_project_stats", {
            path: projectPath,
          });

          // Charger les fichiers non commités depuis le backend
          uncommittedFiles = await invoke<string[]>("get_git_uncommitted_changes", {
            path: projectPath,
          });
        } catch (e) {
          console.error("Erreur chargement Git:", e);
          hasGit = false;
        }
      }

      // Charger l'historique local
      try {
        // Charger l'historique local depuis le backend
        localHistory = await invoke<LocalHistory>("get_local_project_history", {
          path: projectPath,
        });

        // Charger les changements récents depuis le backend
        recentChanges = await invoke<string[]>("get_recent_local_changes", {
          path: projectPath,
        });
      } catch (e) {
        // En cas d'erreur, initialiser un historique local vide
        console.error("Erreur chargement historique local:", e);
        // Historique local vide pour éviter les erreurs d'affichage
        localHistory = {
          events: [],
          total_events: 0,
          last_scan_date: null,
          total_scans: 0,
        };
      }
    } catch (e) {
      // En cas d'erreur critique, afficher un message d'erreur
      error = String(e);
      console.error("Erreur chargement données:", e);
    } finally {
      // Arrêter le chargement en fin de traitement dans tous les cas
      loading = false;
    }
  }

  // Fonction pour basculer l'affichage des fichiers modifiés d'un commit
  function toggleCommit(hash: string) {
    // Basculer l'état d'expansion du commit dans le Set
    if (expandedCommits.has(hash)) {
      expandedCommits.delete(hash); // Supprimer le hash du Set pour le replier
    } else {
      expandedCommits.add(hash); // Ajouter le hash au Set pour l'expanser
    }
    // Créer un nouveau Set pour déclencher la réactivité de Svelte
    expandedCommits = new Set(expandedCommits);
  }

  // Fonction pour basculer l'affichage des fichiers affectés d'un événement local
  function toggleLocalEvent(timestamp: string) {
    // Basculer l'état d'expansion de l'événement local dans le Set
    if (expandedLocalEvents.has(timestamp)) {
      expandedLocalEvents.delete(timestamp); // Supprimer le timestamp du Set pour le replier
    } else {
      expandedLocalEvents.add(timestamp); // Ajouter le timestamp au Set pour l'expanser
    }
    // Créer un nouveau Set pour déclencher la réactivité de Svelte
    expandedLocalEvents = new Set(expandedLocalEvents);
  }

  // Fonction utilitaire pour formater les dates de manière lisible
  function formatDate(dateStr: string): string {
    try {
      const date = new Date(dateStr); // Tenter de créer un objet Date à partir de la chaîne
      // Formater la date 
      return date.toLocaleDateString("fr-FR", {
        day: "2-digit",
        month: "short",
        year: "numeric",
        hour: "2-digit",
        minute: "2-digit",
      });
    } catch {
      return dateStr; // En cas d'erreur de parsing, retourner la chaîne d'origine
    }
  }

  // Fonction pour obtenir une icône en fonction du type d'événement local
  function getEventIcon(eventType: string): string {
    // Retourner une icône différente selon le type d'événement local préconfiguré
    switch (eventType) {
      case "scan":
        return "🔍";
      case "analysis":
        return "📊";
      case "modification":
        return "✏️";
      default:
        return "📝";
    }
  }

  // Fonction pour obtenir une couleur en fonction du type d'événement local
  function getEventColor(eventType: string): string {
    // Retourner une couleur différente selon le type d'événement local préconfiguré
    switch (eventType) {
      case "scan":
        return "#4CAF50";
      case "analysis":
        return "#2196F3";
      case "modification":
        return "#FF9800";
      default:
        return "#9E9E9E";
    }
  }

  // Charger les données à l'initialisation du composant
  onMount(() => {
    loadHistoryData(); // Charger l'historique dès que le composant est monté
  });
</script>

<main class="page-container">
  <!-- En-tête de la page avec un titre et une icône, utilisant une transition de fondu pour l'apparition -->
  <div class="header" transition:fade={{ duration: 300, delay: 100 }}>
    <h1>📊 Données du Projet</h1>
  </div>

  <!-- Contenu principal de la page avec une transition de fondu pour l'apparition -->
  <div class="content-wrapper" transition:fade={{ duration: 300, delay: 200 }}>
    <!-- Contenu de la page -->
    <div class="content">
      <!-- Page de chargement -->
      {#if loading}
        <div class="loading">
          <div class="spinner"></div>
          <p>Chargement de l'historique...</p>
        </div>
        <!-- Affichage d'une erreur si le chargement a échoué -->
      {:else if error}
        <div class="error-message">
          <p>❌ Erreur: {error}</p>
        </div>
      <!-- Affichage de l'historique si le chargement a réussi -->
      {:else}
        <div class="history-grid">
          <!-- Colonne Gauche: Historique Local -->
          <div class="history-column local-history">
            <div class="column-header">
              <h2>📝 Historique Local</h2>
              <!-- Badge affichant le nombre total d'événements locaux -->
              <div class="stats-badge">
                {localHistory?.total_events || 0} événements
              </div>
            </div>
            <!-- Afficher l'historique local si disponible -->
            {#if localHistory && localHistory.events.length > 0}
              <div class="stats-summary">
                <!-- Afficher les statistiques de l'historique local -->
                <div class="stat-item">
                  <span class="stat-label">Total scans:</span>
                  <!-- Afficher le nombre total de scans -->
                  <span class="stat-value">{localHistory.total_scans}</span>
                </div>
                <!-- Afficher la date du dernier scan -->
                {#if localHistory.last_scan_date}
                  <div class="stat-item">
                    <span class="stat-label">Dernier scan:</span>
                    <!-- Formater et afficher la date du dernier scan -->
                    <span class="stat-value">{formatDate(localHistory.last_scan_date)}</span>
                  </div>
                {/if}
                <!-- Afficher les fichiers modifiés récemment -->
                {#if recentChanges.length > 0}
                  <div class="stat-item">
                    <span class="stat-label">Fichiers modifiés récemment:</span>
                    <!-- Afficher le nombre de fichiers modifiés récemment -->
                    <span class="stat-value">{recentChanges.length}</span>
                  </div>
                {/if}
              </div>

              <!-- Liste des événements locaux -->
              <div class="events-list">
                {#each localHistory.events as event}
                  <!-- Carte d'événement local -->
                  <div class="event-card" style="border-left-color: {getEventColor(event.event_type)}">
                    <!-- En-tête de l'événement local avec un bouton pour basculer l'affichage des fichiers affectés -->
                    <div 
                      class="event-header" 
                      role="button" 
                      tabindex="0"
                      onclick={() => toggleLocalEvent(event.timestamp)}
                      onkeydown={(e) => e.key === 'Enter' || e.key === ' ' ? toggleLocalEvent(event.timestamp) : null}
                    >
                      <div class="event-title">
                        <!-- Afficher une icône et le type de l'événement local -->
                        <span class="event-icon">{getEventIcon(event.event_type)}</span>
                        <!-- Afficher le type de l'événement local en majuscules -->
                        <span class="event-type">{event.event_type.toUpperCase()}</span>
                      </div>
                      <!-- Afficher la date de l'événement local formatée -->
                      <span class="event-date">{formatDate(event.timestamp)}</span>
                    </div>
                    <!-- Afficher la description de l'événement local -->
                    <p class="event-description">{event.description}</p>
                    
                    <!-- Afficher les fichiers affectés si l'événement local en a -->
                    {#if event.files_affected.length > 0}
                      <!-- Bouton pour basculer l'affichage des fichiers affectés de l'événement local -->
                      <button
                        class="toggle-btn"
                        onclick={() => toggleLocalEvent(event.timestamp)}
                      >
                        <!-- Afficher une flèche indiquant si la liste des fichiers est développée -->
                        {expandedLocalEvents.has(event.timestamp) ? "▼" : "▶"} 
                        <!-- Afficher le nombre de fichiers affectés par l'événement local -->
                        {event.files_affected.length} fichier(s) affecté(s)
                      </button>
                      
                      <!-- Afficher la liste des fichiers affectés si l'événement local est développé -->
                      {#if expandedLocalEvents.has(event.timestamp)}
                        <!-- Liste des fichiers affectés par l'événement local avec une transition de fondu -->
                        <div class="files-list" transition:fade={{ duration: 200 }}>
                          <!-- Afficher chaque fichier affecté avec une icône de document -->
                          {#each event.files_affected as file}
                            <div class="file-item">📄 {file}</div>
                          {/each}
                        </div>
                      {/if}
                    {/if}
                  </div>
                {/each}
              </div>
            {:else}
              <!-- Afficher un état vide si aucun événement local n'est enregistré -->
              <div class="empty-state">
                <p>📭 Aucun événement local enregistré</p>
                <p class="empty-hint">Les scans et analyses seront enregistrés ici</p>
              </div>
            {/if}
          </div>

          <!-- Colonne Droite: Historique Git -->
          <div class="history-column git-history">
            <div class="column-header">
              <h2>🌿 Historique Git</h2>
              <!-- Si c'est un dépôt Git et que les statistiques sont disponibles -->
              {#if hasGit && gitStats}
                <!-- Badge affichant le nombre total de commits Git -->
                <div class="stats-badge">
                  {gitStats.total_commits} commits
                </div>
              {/if}
            </div>

            <!-- Afficher l'historique Git si c'est un dépôt Git et que des commits sont disponibles -->
            {#if !hasGit}
              <div class="empty-state">
                <!-- Afficher un message d'erreur si ce n'est pas un dépôt Git -->
                <p>❌ Ce projet n'est pas un dépôt Git</p>
                <p class="empty-hint">Initialisez un dépôt Git pour voir l'historique des commits</p>
              </div>
            <!-- Si l'historique Git est disponible et que des commits sont présents -->
            {:else if gitHistory && gitHistory.commits.length > 0}
              <!-- Afficher les statistiques Git si disponibles -->
              {#if gitStats}
                <!-- Résumé des statistiques Git du projet -->
                <div class="stats-summary">
                  <div class="stat-item">
                    <span class="stat-label">Contributeurs:</span>
                    <!-- Afficher le nombre total de contributeurs Git -->
                    <span class="stat-value">{gitStats.total_contributors}</span>
                  </div>
                  <div class="stat-item">
                    <span class="stat-label">Branches:</span>
                    <!-- Afficher le nombre total de branches Git -->
                    <span class="stat-value">{gitStats.total_branches}</span>
                  </div>
                  <div class="stat-item">
                    <span class="stat-label">Premier commit:</span>
                    <!-- Formater et afficher la date du premier commit Git -->
                    <span class="stat-value">{formatDate(gitStats.first_commit_date)}</span>
                  </div>
                  {#if uncommittedFiles.length > 0}
                    <div class="stat-item warning">
                      <span class="stat-label">⚠️ Fichiers non commités:</span>
                      <!-- Afficher le nombre de fichiers non commités avec une mise en évidence d'avertissement -->
                      <span class="stat-value">{uncommittedFiles.length}</span>
                    </div>
                  {/if}
                </div>
              {/if}

              <!-- Liste des commits Git -->
              <div class="commits-list">
                {#each gitHistory.commits as commit}
                  <!-- Carte de commit Git -->
                  <div class="commit-card">
                    <!-- En-tête du commit avec un bouton pour basculer l'affichage des fichiers modifiés -->
                    <div 
                      class="commit-header" 
                      role="button" 
                      tabindex="0"
                      onclick={() => toggleCommit(commit.hash)}
                      onkeydown={(e) => e.key === 'Enter' || e.key === ' ' ? toggleCommit(commit.hash) : null}
                    >
                      <div class="commit-info">
                        <!-- Afficher le hash du commit avec une mise en forme monospace -->
                        <span class="commit-hash">#{commit.hash}</span>
                        <!-- Afficher l'auteur du commit avec une icône d'utilisateur -->
                        <span class="commit-author">👤 {commit.author}</span>
                      </div>
                      <!-- Afficher la date du commit formatée -->
                      <span class="commit-date">{formatDate(commit.date)}</span>
                    </div>
                    
                    <!-- Afficher le message du commit -->
                    <p class="commit-message">{commit.message}</p>
                    
                    <!-- Afficher les statistiques du commit (insertions, suppressions, fichiers modifiés) -->
                    <div class="commit-stats">
                      <span class="stat-add">+{commit.insertions}</span>
                      <span class="stat-del">-{commit.deletions}</span>
                      <span class="stat-files">📁 {commit.files_changed.length}</span>
                    </div>

                    <!-- Afficher un bouton pour voir les fichiers modifiés si le commit en a -->
                    {#if commit.files_changed.length > 0}
                      <button
                        class="toggle-btn"
                        onclick={() => toggleCommit(commit.hash)}
                      >
                        <!-- Afficher une flèche indiquant si la liste des fichiers modifiés est développée -->
                        {expandedCommits.has(commit.hash) ? "▼" : "▶"} 
                        Voir les fichiers modifiés
                      </button>
                      
                      <!-- Afficher la liste des fichiers modifiés si le commit est développé -->
                      {#if expandedCommits.has(commit.hash)}
                        <div class="files-list" transition:fade={{ duration: 200 }}>
                          <!-- Afficher chaque fichier modifié avec une icône de document -->
                          {#each commit.files_changed as file}
                            <div class="file-item">📄 {file}</div>
                          {/each}
                        </div>
                      {/if}
                    {/if}
                  </div>
                {/each}
              </div>
            {:else}
              <!-- Afficher un état vide si aucun commit Git n'est trouvé -->
              <div class="empty-state">
                <p>📭 Aucun commit trouvé</p>
                <p class="empty-hint">Le dépôt Git est vide ou non initialisé</p>
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
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
    max-width: 1600px;
    margin: 0 auto;
  }

  .loading {
    text-align: center;
    padding: 60px 20px;
  }

  .spinner {
    border: 4px solid rgba(103, 126, 234, 0.3);
    border-top: 4px solid #667eea;
    border-radius: 50%;
    width: 50px;
    height: 50px;
    animation: spin 1s linear infinite;
    margin: 0 auto 20px;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-message {
    padding: 20px;
    background: rgba(244, 67, 54, 0.1);
    border: 1px solid rgba(244, 67, 54, 0.5);
    border-radius: 12px;
    color: #ff5252;
    text-align: center;
  }

  .history-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 30px;
  }

  .history-column {
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    padding: 25px;
    border-radius: 16px;
    border: 1px solid rgba(103, 126, 234, 0.3);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(10px);
  }

  .column-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    padding-bottom: 15px;
    border-bottom: 2px solid rgba(103, 126, 234, 0.3);
  }

  .column-header h2 {
    color: #667eea;
    font-size: 1.5rem;
    margin: 0;
    font-weight: 600;
  }

  .stats-badge {
    background: rgba(103, 126, 234, 0.2);
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 0.85rem;
    color: #667eea;
    font-weight: 600;
  }

  .stats-summary {
    background: rgba(103, 126, 234, 0.1);
    padding: 15px;
    border-radius: 10px;
    margin-bottom: 20px;
    border: 1px solid rgba(103, 126, 234, 0.2);
  }

  .stat-item {
    display: flex;
    justify-content: space-between;
    padding: 8px 0;
    border-bottom: 1px solid rgba(103, 126, 234, 0.1);
  }

  .stat-item:last-child {
    border-bottom: none;
  }

  .stat-item.warning {
    background: rgba(255, 152, 0, 0.1);
    padding: 8px 10px;
    border-radius: 6px;
    margin-top: 5px;
  }

  .stat-label {
    color: #b0b0b0;
    font-size: 0.9rem;
  }

  .stat-value {
    color: #667eea;
    font-weight: 600;
  }

  .events-list,
  .commits-list {
    max-height: 600px;
    overflow-y: auto;
    padding-right: 10px;
  }

  .events-list::-webkit-scrollbar,
  .commits-list::-webkit-scrollbar {
    width: 8px;
  }

  .events-list::-webkit-scrollbar-thumb,
  .commits-list::-webkit-scrollbar-thumb {
    background: rgba(103, 126, 234, 0.5);
    border-radius: 4px;
  }

  .event-card,
  .commit-card {
    background: rgba(17, 17, 34, 0.6);
    padding: 18px;
    border-radius: 12px;
    margin-bottom: 15px;
    border-left: 4px solid #667eea;
    transition: all 0.3s ease;
  }

  .event-card:hover,
  .commit-card:hover {
    transform: translateX(5px);
    box-shadow: 0 4px 16px rgba(103, 126, 234, 0.3);
  }

  .event-header,
  .commit-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
    cursor: pointer;
  }

  .event-title {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .event-icon {
    font-size: 1.3rem;
  }

  .event-type {
    color: #667eea;
    font-weight: 600;
    font-size: 0.85rem;
  }

  .event-date,
  .commit-date {
    color: #888;
    font-size: 0.85rem;
  }

  .event-description,
  .commit-message {
    color: #e0e0e0;
    margin: 10px 0;
    line-height: 1.5;
  }

  .commit-info {
    display: flex;
    gap: 15px;
    align-items: center;
  }

  .commit-hash {
    font-family: monospace;
    color: #667eea;
    font-weight: 600;
    font-size: 0.9rem;
  }

  .commit-author {
    color: #b0b0b0;
    font-size: 0.9rem;
  }

  .commit-stats {
    display: flex;
    gap: 15px;
    margin: 10px 0;
    font-size: 0.9rem;
  }

  .stat-add {
    color: #4CAF50;
    font-weight: 600;
  }

  .stat-del {
    color: #f44336;
    font-weight: 600;
  }

  .stat-files {
    color: #2196F3;
    font-weight: 600;
  }

  .toggle-btn {
    background: rgba(103, 126, 234, 0.2);
    border: 1px solid rgba(103, 126, 234, 0.4);
    color: #667eea;
    padding: 8px 14px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.3s ease;
    width: 100%;
    text-align: left;
    margin-top: 10px;
  }

  .toggle-btn:hover {
    background: rgba(103, 126, 234, 0.3);
    border-color: #667eea;
  }

  .files-list {
    margin-top: 10px;
    padding: 10px;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 8px;
    max-height: 200px;
    overflow-y: auto;
  }

  .file-item {
    padding: 6px 10px;
    color: #e0e0e0;
    font-size: 0.85rem;
    border-bottom: 1px solid rgba(103, 126, 234, 0.1);
  }

  .file-item:last-child {
    border-bottom: none;
  }

  .empty-state {
    text-align: center;
    padding: 60px 20px;
    color: #888;
  }

  .empty-state p {
    font-size: 1.1rem;
    margin: 10px 0;
  }

  .empty-hint {
    font-size: 0.9rem;
    color: #666;
    font-style: italic;
  }

  @media (max-width: 1200px) {
    .history-grid {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 768px) {
    .header h1 {
      font-size: 1.5rem;
    }

    .content-wrapper {
      padding: 80px 15px 15px;
    }

    .history-column {
      padding: 15px;
    }

    .column-header {
      flex-direction: column;
      gap: 10px;
      align-items: flex-start;
    }
  }
</style>
