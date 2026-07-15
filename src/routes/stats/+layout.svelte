<script lang="ts">
  import VerticalToolbar from "./VerticalToolbar.svelte";
  import PageTransition from "$lib/components/PageTransition.svelte";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

  let { children } = $props();
  let projectPath = $state("");
  let showTransition = $state(false);

  onMount(() => {
    const params = new URLSearchParams(window.location.search);
    projectPath = params.get("path") || "";
  });

  async function backToDashboard() {
    try {
      // Afficher l'écran de transition
      showTransition = true;
      
      // Attendre un peu pour que l'animation se lance
      await new Promise(resolve => setTimeout(resolve, 300));
      
      // Appeler la commande Rust pour redimensionner et naviguer
      await invoke("back_to_dashboard");
      
      // Attendre un peu avant de naviguer
      await new Promise(resolve => setTimeout(resolve, 200));
    } catch (error) {
      console.error("Erreur lors du redimensionnement:", error);
      // Naviguer quand même même si le redimensionnement échoue
      await goto("/");
    }
  }
</script>

<div class="stats-layout">
  <VerticalToolbar projectPath={projectPath} onBackToDashboard={backToDashboard} />
  <div class="main-content">
    {@render children()}
  </div>
</div>

<PageTransition show={showTransition} />

<style>
  :global(html),
  :global(body) {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .stats-layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    max-height: 100vh;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    overflow: hidden;
    position: fixed;
    top: 0;
    left: 0;
  }

  .main-content {
    flex: 1;
    margin-left: 70px;
    height: 100vh;
    max-height: 100vh;
    overflow-y: auto;
    overflow-x: hidden;
  }
</style>
