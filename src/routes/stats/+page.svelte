<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { fade } from "svelte/transition";
  import { Chart } from "chart.js/auto";
  import { invoke } from "@tauri-apps/api/core";

  // Variables d'état pour stocker les données et les références aux éléments du DOM
  let projectPath = $state("");
  let stats = $state<any>(null);
  let projectContent = $state<any[]>([]);
  let modifiedFiles = $state<any[]>([]);
  let colorPalette = $state<Record<string, { bg: string; border: string }>>({});
  let errorMessage = $state("");
  let languageChartCanvas = $state<HTMLCanvasElement | null>(null);
  let evolutionChartCanvas = $state<HTMLCanvasElement | null>(null);
  let filesChartCanvas = $state<HTMLCanvasElement | null>(null);
  let languageChart: Chart | null = null;
  let evolutionChart: Chart | null = null;
  let filesChart: Chart | null = null;
  let isInitialized = $state(false);

  // Fonction pour obtenir la couleur d'une catégorie/langage
  function getColor(name: string): { bg: string; border: string } {
    return colorPalette[name] || colorPalette["default"] || { 
      bg: "rgba(149, 165, 166, 0.8)", 
      border: "rgba(149, 165, 166, 1)" 
    };
  }

  // Fonction pour naviguer vers l'onglet structure avec le chemin du fichier
  function navigateToStructure(filePath: string) {
    const params = new URLSearchParams(window.location.search);
    const projectPath = params.get("path") || "";
    window.location.href = `/stats/structure?path=${encodeURIComponent(projectPath)}&file=${encodeURIComponent(filePath)}`;
  }

  // Fonction pour formater les octets en unité adaptée
  function formatBytes(bytes: number): string {
    if (bytes === 0) return '0 o';
    
    const k = 1024;
    const sizes = ['o', 'Ko', 'Mo', 'Go', 'To'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  }

  // Fonction pour détruire proprement les graphiques
  function destroyCharts() {
    if (languageChart) {
      languageChart.destroy();
      languageChart = null;
    }
    if (evolutionChart) {
      evolutionChart.destroy();
      evolutionChart = null;
    }
    if (filesChart) {
      filesChart.destroy();
      filesChart = null;
    }
  }

  $effect(() => {
    // Récupérer le chemin du projet depuis les query params
    const params = new URLSearchParams(window.location.search);
    const newProjectPath = params.get("path") || "";
    
    // Ne recharger que si le chemin a changé ou si ce n'est pas initialisé
    if (newProjectPath !== projectPath || !isInitialized) {
      projectPath = newProjectPath;
      
      // Détruire les graphiques existants avant de recharger
      destroyCharts();
      stats = null;
      projectContent = [];
      modifiedFiles = [];
      
      // Charger la palette de couleurs depuis le backend
      invoke<Record<string, { bg: string; border: string }>>("get_colors")
        // Stocker la palette de couleurs dans l'état
        .then(colors => {
          colorPalette = colors;
        })
        // Gérer les erreurs lors du chargement de la palette de couleurs
        .catch(err => {
          console.error("Erreur lors du chargement de la palette:", err);
        });
      
      if (projectPath) {
        loadProjectStats();
        isInitialized = true;
      }
    }
  });

  onMount(() => {
    // Gérer le redimensionnement de la fenêtre
    const handleResize = () => {
      // Redimensionner les graphiques si les canvas existent
      if (languageChart) languageChart.resize();
      if (evolutionChart) evolutionChart.resize();
      if (filesChart) filesChart.resize();
    };

    // Ajouter un écouteur d'événement pour le redimensionnement de la fenêtre
    window.addEventListener('resize', handleResize);

    // Nettoyer l'écouteur d'événement lors de la destruction du composant
    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });

  onDestroy(() => {
    // Nettoyer les graphiques à la destruction du composant
    destroyCharts();
  });

  async function loadProjectStats() {
    try {
      errorMessage = "";
      
      // Appeler les fonctions Rust pour obtenir les vraies données du projet
      const [languagesData, modificationsData, contentData, filesData] = await Promise.all([
        invoke<Record<string, number>>("analyze_project_languages", { path: projectPath }),
        invoke<Array<{ date: string; bytes: number }>>("analyze_project_modifications", { path: projectPath }),
        invoke<Array<{ category: string; files: Array<{ extension: string; count: number }>; total: number }>>("analyze_project_content", { path: projectPath }),
        invoke<Array<{ file: string; modifications: number }>>("analyze_modified_files", { path: projectPath })
      ]);

      stats = {
        name: projectPath.split(/[\\\/]/).pop(),
        path: projectPath,
        languages: languagesData,
        modifications: modificationsData
      };
      
      projectContent = contentData;
      modifiedFiles = filesData;

      // Attendre que le DOM soit mis à jour
      await new Promise(resolve => setTimeout(resolve, 150));
      
      // Vérifier que les canvas existent avant de créer les graphiques
      if (languageChartCanvas && evolutionChartCanvas && filesChartCanvas) {
        createContentChart();
        createEvolutionChart();
        createFilesChart();
      }
    } catch (error) {
      errorMessage = `Erreur: ${error}`;
      console.error("Erreur lors du chargement des statistiques:", error);
    }
  }

  function createContentChart() {
    if (languageChart) {
      languageChart.destroy();
    }

    const ctx = languageChartCanvas?.getContext("2d");
    if (!ctx || !projectContent || projectContent.length === 0) return;

    // Préparer les données pour le camembert
    const labels: string[] = [];
    const data: number[] = [];
    const backgroundColors: string[] = [];
    const borderColors: string[] = [];

    projectContent.forEach(category => {
      // Créer le label avec les détails des extensions
      const extensionDetails = category.files
        .map((f: { extension: string; count: number }) => `${f.count} .${f.extension}`)
        .join(", ");
      
      labels.push(category.category);
      data.push(category.total);
      
      const color = getColor(category.category);
      backgroundColors.push(color.bg);
      borderColors.push(color.border);
    });

    languageChart = new Chart(ctx, {
      type: "pie",
      data: {
        labels: labels,
        datasets: [{
          data: data,
          backgroundColor: backgroundColors,
          borderColor: borderColors,
          borderWidth: 2,
          hoverOffset: 25,
          hoverBorderWidth: 3,
          hoverBorderColor: "#ffffff"
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        interaction: {
          mode: 'index',
          intersect: true
        },
        plugins: {
          legend: {
            position: "bottom",
            labels: {
              color: "#ffffff",
              font: {
                size: 13,
                family: "Inter, sans-serif"
              },
              padding: 15,
              generateLabels: function(chart) {
                const data = chart.data;
                if (data.labels?.length && data.datasets.length) {
                  return data.labels.map((label, i) => {
                    const category = projectContent[i];
                    const extensionDetails = category.files
                      .map((f: { extension: string; count: number }) => `${f.count} .${f.extension}`)
                      .join(", ");
                    
                    const bgColors = data.datasets[0].backgroundColor;
                    const fillStyle = Array.isArray(bgColors) ? bgColors[i] as string : bgColors as string;
                    
                    return {
                      text: `${label} (${extensionDetails})`,
                      fillStyle: fillStyle,
                      fontColor: "#ffffff",
                      hidden: false,
                      index: i
                    };
                  });
                }
                return [];
              }
            },
            onHover: function(event: any, legendItem: any, legend: any) {
              const chart = legend.chart;
              const index = legendItem.index;
              
              // Modifier les couleurs pour créer l'effet de focus
              chart.data.datasets[0].backgroundColor = backgroundColors.map((color: string, i: number) => {
                if (i === index) {
                  // Accentuer la couleur de la part survolée
                  return color.replace(/[\d.]+\)$/g, '1)');
                } else {
                  // Atténuer les autres
                  return color.replace(/[\d.]+\)$/g, '0.3)');
                }
              });
              chart.update();
            },
            onLeave: function(event: any, legendItem: any, legend: any) {
              const chart = legend.chart;
              // Restaurer les couleurs originales
              chart.data.datasets[0].backgroundColor = backgroundColors;
              chart.update();
            }
          },
          tooltip: {
            callbacks: {
              label: function(context) {
                const category = projectContent[context.dataIndex];
                const extensionDetails = category.files
                  .map((f: { extension: string; count: number }) => `${f.count} .${f.extension}`)
                  .join(", ");
                return `${context.label}: ${extensionDetails}`;
              }
            }
          }
        },
        onHover: function(event: any, activeElements: any[]) {
          if (activeElements.length > 0) {
            const index = activeElements[0].index;
            
            // Modifier les couleurs pour créer l'effet de focus
            languageChart!.data.datasets[0].backgroundColor = backgroundColors.map((color: string, i: number) => {
              if (i === index) {
                // Accentuer la couleur de la part survolée
                return color.replace(/[\d.]+\)$/g, '1)');
              } else {
                // Atténuer les autres
                return color.replace(/[\d.]+\)$/g, '0.3)');
              }
            });
            languageChart!.update('none');
          } else {
            // Restaurer les couleurs originales
            languageChart!.data.datasets[0].backgroundColor = backgroundColors;
            languageChart!.update('none');
          }
        }
      }
    });
  }

  // Fonction pour créer le graphique d'évolution des modifications
  function createEvolutionChart() {
    if (evolutionChart) {
      evolutionChart.destroy();
    }

    const ctx = evolutionChartCanvas?.getContext("2d");
    if (!ctx || !stats) return;

    const dates = stats.modifications.map((m: any) => m.date);
    const bytes = stats.modifications.map((m: any) => m.bytes);

    evolutionChart = new Chart(ctx, {
      type: "line",
      data: {
        labels: dates,
        datasets: [{
          label: "Modifications (octets)",
          data: bytes,
          borderColor: "rgba(103, 126, 234, 1)",
          backgroundColor: "rgba(103, 126, 234, 0.1)",
          borderWidth: 3,
          fill: true,
          tension: 0.4,
          pointRadius: 4,
          pointHoverRadius: 6,
          pointBackgroundColor: "rgba(103, 126, 234, 1)",
          pointBorderColor: "#fff",
          pointBorderWidth: 2
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            display: true,
            position: "top",
            labels: {
              color: "#e0e0e0",
              font: {
                size: 13,
                family: "Inter, sans-serif"
              }
            }
          },
          tooltip: {
            callbacks: {
              label: function(context) {
                const value = context.parsed.y ?? 0;
                return `Modifications: ${formatBytes(value)}`;
              }
            }
          }
        },
        scales: {
          y: {
            beginAtZero: true,
            ticks: {
              color: "#e0e0e0",
              callback: function(value) {
                return formatBytes(value as number);
              }
            },
            grid: {
              color: "rgba(255, 255, 255, 0.1)"
            }
          },
          x: {
            ticks: {
              color: "#e0e0e0"
            },
            grid: {
              color: "rgba(255, 255, 255, 0.1)"
            }
          }
        }
      }
    });
  }

  // Fonction pour créer le graphique des fichiers modifiés
  function createFilesChart() {
    if (filesChart) {
      filesChart.destroy();
    }

    const ctx = filesChartCanvas?.getContext("2d");
    if (!ctx || !modifiedFiles || modifiedFiles.length === 0) return;

    // Prendre les 10 fichiers les plus modifiés
    const topFiles = modifiedFiles.slice(0, 10);
    const fileNames = topFiles.map((f: any) => f.file.split(/[\\\/]/).pop());
    const modifications = topFiles.map((f: any) => f.modifications);

    // Variable pour suivre l'index du label survolé
    let hoveredLabelIndex = -1;

    // Plugin personnalisé pour souligner les labels au survol
    const underlinePlugin = {
      id: 'underlineLabels',
      afterDraw: (chart: any) => {
        if (hoveredLabelIndex >= 0) {
          const ctx = chart.ctx;
          const yAxis = chart.scales.y;
          const labelY = yAxis.getPixelForValue(hoveredLabelIndex);
          const labelX = yAxis.left;
          const labelWidth = yAxis.width;

          ctx.save();
          ctx.strokeStyle = '#667eea';
          ctx.lineWidth = 2;
          ctx.beginPath();
          ctx.moveTo(labelX, labelY + 8);
          ctx.lineTo(labelX + labelWidth, labelY + 8);
          ctx.stroke();
          ctx.restore();
        }
      }
    };

    filesChart = new Chart(ctx, {
      type: "bar",
      data: {
        labels: fileNames,
        datasets: [{
          label: "Nombre de modifications",
          data: modifications,
          backgroundColor: "rgba(103, 126, 234, 0.8)",
          borderColor: "rgba(103, 126, 234, 1)",
          borderWidth: 2
        }]
      },
      options: {
        indexAxis: 'y',
        responsive: true,
        maintainAspectRatio: false,
        onClick: (event: any, elements: any[]) => {
          if (elements.length > 0) {
            const index = elements[0].index;
            const fullPath = topFiles[index].file;
            navigateToStructure(fullPath);
          }
        },
        onHover: (event: any, elements: any[]) => {
          const canvas = filesChartCanvas;
          if (canvas) {
            canvas.style.cursor = elements.length > 0 ? 'pointer' : 'default';
          }
          
          // Mettre à jour l'index du label survolé
          if (elements.length > 0) {
            hoveredLabelIndex = elements[0].index;
          } else {
            hoveredLabelIndex = -1;
          }
          filesChart?.update('none');
        },
        plugins: {
          legend: {
            display: false
          },
          tooltip: {
            callbacks: {
              label: function(context) {
                return `Taille: ${formatBytes(context.parsed.x as number)}`;
              },
              afterLabel: function() {
                return 'Cliquer pour voir dans la structure';
              }
            }
          }
        },
        scales: {
          x: {
            beginAtZero: true,
            ticks: {
              color: "#e0e0e0",
              callback: function(value) {
                return formatBytes(value as number);
              }
            },
            grid: {
              color: "rgba(255, 255, 255, 0.1)"
            }
          },
          y: {
            ticks: {
              color: "#e0e0e0",
              font: {
                size: 11
              }
            },
            grid: {
              display: false
            }
          }
        }
      },
      plugins: [underlinePlugin]
    });
  }
</script>

<main class="stats-container">
  <!-- En-tête fixe avec le titre de la page -->
  <div class="header" transition:fade={{ duration: 300 }}>
    <h1>📈 Statistiques</h1>
  </div>

  <!-- Contenu principal avec les graphiques et un effet de transition -->
  <div class="content-wrapper" transition:fade={{ duration: 300, delay: 100 }}>
    <!-- Afficher le message d'erreur s'il existe, sinon afficher les graphiques -->
    {#if errorMessage}
      <!-- Message d'erreur stylisé -->
      <div class="error-message">{errorMessage}</div>
    {:else if stats}
      <div class="stats-content">
        <!-- Diagramme camembert du contenu du projet -->
        <div class="chart-card">
          <div class="chart-title">Contenu du projet</div>
          <!-- Canvas pour le graphique camembert -->
          <div class="chart-container">
            <canvas bind:this={languageChartCanvas}></canvas>
          </div>
        </div>

        <!-- Diagramme d'évolution des modifications -->
        <div class="chart-card chart-split">
          <div class="chart-section">
            <div class="chart-title">Évolution des modifications</div>
            <div class="chart-container chart-half-height">
              <canvas bind:this={evolutionChartCanvas}></canvas>
            </div>
          </div>
          <!-- Diagramme des fichiers les plus modifiés -->
          <div class="chart-section">
            <div class="chart-title">Fichiers les plus modifiés</div>
            <!-- Canvas pour le graphique des fichiers modifiés -->
            <div class="chart-container chart-half-height">
              <canvas bind:this={filesChartCanvas}></canvas>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>
</main>

<style>
  /* Styles globaux et de base pour la page de statistiques */
  * {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  /* Style de base pour le corps de la page */
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

  /* Conteneur principal de la page de statistiques */
  .stats-container {
    width: 100%;
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background: transparent;
  }

  /* En-tête fixe avec un fond semi-transparent et un effet de flou */
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

  /* Style du titre de l'en-tête */
  .header h1 {
    color: #667eea;
    font-size: 2rem;
    margin: 0;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-shadow: 0 2px 10px rgba(103, 126, 234, 0.5);
  }

  /* Conteneur du contenu principal */
  .content-wrapper {
    flex: 1;
    padding: 100px 20px 20px; /* Padding pour éviter le chevauchement avec l'en-tête */
    overflow-y: auto;
    overflow-x: hidden;
  }

  /* Grille pour les graphiques avec un espacement et une disposition responsive */
  .stats-content {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
    padding-bottom: 20px;
  }

  /* Style des cartes contenant les graphiques avec un fond semi-transparent et un effet de flou */
  .chart-card {
    display: flex;
    flex-direction: column;
    background: linear-gradient(135deg, rgba(17, 17, 34, 0.9) 0%, rgba(33, 33, 66, 0.9) 100%);
    padding: 25px;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    border: 1px solid rgba(103, 126, 234, 0.3);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
    min-height: 400px;
    height: calc(100vh - 180px);
    max-height: calc(100vh - 180px);
  }

  /* Effet de survol pour les cartes des graphiques */
  .chart-card:hover {
    transform: translateY(-5px);
    box-shadow: 0 12px 40px rgba(103, 126, 234, 0.4);
    border-color: rgba(103, 126, 234, 0.5);
  }

  /* Style de la carte qui contient les deux graphiques d'évolution et de fichiers modifiés */
  .chart-card.chart-split {
    gap: 20px;
    padding: 20px;
  }

  /* Sections individuelles pour les graphiques d'évolution et de fichiers modifiés */
  .chart-section {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
    overflow: hidden;
  }

  /* Style des titres des sections individuelles */
  .chart-section .chart-title {
    margin-bottom: 12px;
    font-size: 1.1rem;
    flex-shrink: 0;
  }

  /* Conteneur pour les graphiques d'évolution et de fichiers modifiés */
  .chart-half-height {
    flex: 1;
    min-height: 0;
    position: relative;
  }

  /* Style des titres des graphiques */
  .chart-title {
    font-size: 1.3rem;
    color: #667eea;
    margin-bottom: 20px;
    font-weight: 600;
    text-align: center;
    letter-spacing: 0.3px;
    flex-shrink: 0;
  }

  /* Conteneur pour les graphiques */
  .chart-container {
    position: relative;
    width: 100%;
    flex: 1;
    min-height: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  /* Style pour les canvas des graphiques pour les rendre responsives */
  .chart-container canvas {
    max-width: 100%;
    max-height: 100%;
  }

  /* Style des messages d'erreur */
  .error-message {
    background: linear-gradient(135deg, rgba(62, 31, 31, 0.9) 0%, rgba(92, 46, 46, 0.9) 100%);
    color: #ff6b6b;
    border: 1px solid rgba(255, 107, 107, 0.3);
    box-shadow: 0 8px 32px rgba(255, 107, 107, 0.2);
    padding: 20px;
    border-radius: 12px;
    text-align: center;
  }

  /* Styles spécifiques pour les écrans moyens */
  @media (max-width: 1200px) {
    .stats-content {
      grid-template-columns: 1fr;
    }

    .chart-card {
      height: calc(100vh - 160px);
      max-height: calc(100vh - 160px);
    }
  }

  /* Styles spécifiques pour les très petits écrans */
  @media (max-width: 768px) {
    .header {
      padding: 15px 20px;
    }

    .header h1 {
      font-size: 1.5rem;
    }

    .content-wrapper {
      padding: 80px 10px 10px;
    }

    .stats-content {
      gap: 15px;
    }

    .chart-card {
      padding: 15px;
      min-height: 350px;
      height: calc(100vh - 140px);
      max-height: calc(100vh - 140px);
    }

    .chart-title {
      font-size: 1.1rem;
      margin-bottom: 15px;
    }

    .chart-section .chart-title {
      font-size: 1rem;
      margin-bottom: 10px;
    }
  }

  /* Styles spécifiques pour les très petits écrans (mobile) */
  @media (max-width: 480px) {
    .header {
      padding: 12px 15px;
    }

    .header h1 {
      font-size: 1.2rem;
    }

    .content-wrapper {
      padding: 70px 8px 8px;
    }

    .stats-content {
      gap: 12px;
    }

    .chart-card {
      padding: 12px;
      min-height: 300px;
      height: calc(100vh - 120px);
      max-height: calc(100vh - 120px);
    }

    .chart-title {
      font-size: 1rem;
      margin-bottom: 12px;
    }

    .chart-section .chart-title {
      font-size: 0.95rem;
      margin-bottom: 8px;
    }
  }
</style>
