<script lang="ts">
  import { goto } from "$app/navigation";

  /* Gérer les propriétés passées au composant */
  let { projectPath = "", onBackToDashboard } = $props<{ 
    projectPath: string;
    onBackToDashboard?: () => Promise<void>;
  }>();
  
  let clickedItem = $state<string | null>(null); // État pour gérer l'animation de clic

  /* Navigue vers la page principale du tableau de bord */
  async function navigateHome() {
    setTimeout(() => clickedItem = null, 200); // Réinitialise l'état "clicked" après un court délai
    clickedItem = "home";
    if (onBackToDashboard) {
      await onBackToDashboard();
    }
  }

  /* Navigue vers la page spécifiée en mettant à jour l'URL */
  async function navigateTo(path: string) {
    clickedItem = path;
    setTimeout(() => clickedItem = null, 200); // Réinitialise l'état "clicked" après un court délai
    
    // Si c'est "statistiques", naviguer vers /stats (la page racine)
    if (path === "statistiques") {
      await goto(`/stats?path=${encodeURIComponent(projectPath)}`);
    } else {
      await goto(`/stats/${path}?path=${encodeURIComponent(projectPath)}`);
    }
  }

  /* Détermine si le bouton correspondant au chemin donné doit être actif */
  function isActive(path: string): boolean {
    const currentPath = window.location.pathname;
    
    // Pour "statistiques", actif si on est exactement sur /stats
    if (path === "statistiques") {
      return currentPath === "/stats" || currentPath === "/stats/";
    }
    
    // Pour les autres pages, vérifier qu'on est sur la sous-route spécifique
    return currentPath.includes(`/stats/${path}`);
  }

</script>

<nav 
  class="vertical-toolbar" 
>
  <button 
    class="nav-item" 
    class:clicked={clickedItem === "home"}
    onclick={navigateHome}
    title="Retour au menu principal"
  >
    <div class="icon">🏠</div>
  </button>

  <div class="separator"></div>

  <button 
    class="nav-item" 
    class:active={isActive("statistiques")}
    class:clicked={clickedItem === "statistiques"}
    onclick={() => navigateTo("statistiques")}
    title="Statistiques du projet"
  >
    <div class="icon">📈</div>
  </button>

  <button 
    class="nav-item" 
    class:active={isActive("infos")}
    class:clicked={clickedItem === "infos"}
    onclick={() => navigateTo("infos")}
    title="Informations"
  >
    <div class="icon">ℹ️</div>
  </button>

  <button 
    class="nav-item" 
    class:active={isActive("donnees")}
    class:clicked={clickedItem === "donnees"}
    onclick={() => navigateTo("donnees")}
    title="Données du projet"
  >
    <div class="icon">📊</div>
  </button>

  <button 
    class="nav-item" 
    class:active={isActive("structure")}
    class:clicked={clickedItem === "structure"}
    onclick={() => navigateTo("structure")}
    title="Structure du projet"
  >
    <div class="icon">🗂️</div>
  </button>
</nav>

<style>
  .vertical-toolbar {
    position: fixed;
    left: 0;
    top: 0;
    height: 100vh;
    max-height: 100vh;
    background: #1a1a2e;
    border-right: 2px solid #333;
    display: flex;
    flex-direction: column;
    padding: 15px 0;
    gap: 8px;
    width: 70px;
    transition: width 0.3s ease;
    z-index: 1000;
    overflow-y: auto;
    overflow-x: hidden;
    box-sizing: border-box;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 15px;
    padding: 12px 15px;
    background: transparent;
    border: none;
    color: #e0e0e0;
    cursor: pointer;
    transition: all 0.3s ease;
    text-align: left;
    width: 100%;
    border-left: 3px solid transparent;
    position: relative;
    overflow: hidden;
  }

  /* Effet de vague bleue au clic */
  .nav-item::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(102, 126, 234, 0.6), transparent);
    transition: left 0.6s ease;
  }

  .nav-item.clicked::before {
    left: 100%;
  }

  /* Survol d'un bouton de navigation */
  .nav-item:hover {
    border-left-color: rgba(255, 155, 25, 0.3);
    background: linear-gradient(135deg, rgba(255, 155, 24, 0.2) 0%, rgba(74, 24, 255, 0.3) 100%);
  }
  
  .nav-item:hover .icon {
    transform: scale(1.2);
  }

  /* Bouton de navigation actif */
  .nav-item.active {
    border-left-color: rgba(255, 155, 25, 0.9);
    background: linear-gradient(135deg, rgba(255, 155, 24, 0.7) 0%, rgba(74, 24, 255, 0.7) 100%);
  }

  .icon {
    font-size: 24px;
    min-width: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.1s ease-in-out;
    position: relative;
    z-index: 1;
  }

  .separator {
    height: 1px;
    background: #333;
    margin: 8px 15px;
  }
</style>
