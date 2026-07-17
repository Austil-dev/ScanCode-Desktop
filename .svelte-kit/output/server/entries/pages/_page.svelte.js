import { x as attr, y as ensure_array_like } from "../../chunks/index2.js";
import "@tauri-apps/plugin-dialog";
import "@tauri-apps/api/core";
import "@tauri-apps/api/webviewWindow";
import { o as onDestroy } from "../../chunks/index-server.js";
import { P as PageTransition } from "../../chunks/PageTransition.js";
import { k as escape_html } from "../../chunks/escaping.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let projectPath = "";
    let isLoading = false;
    let recentProjects = [];
    let windowWidth = typeof window !== "undefined" ? window.innerWidth : 1024;
    let showTransition = false;
    onDestroy(() => {
    });
    function formatDate(timestamp) {
      const date = new Date(timestamp);
      return date.toLocaleDateString("fr-FR", {
        year: "numeric",
        month: "2-digit",
        day: "2-digit",
        hour: "2-digit",
        minute: "2-digit"
      });
    }
    function truncate(text) {
      const maxWidth = Math.max(windowWidth * 0.8, 300);
      const estimatedCharWidth = 8;
      const maxChars = Math.floor(maxWidth / estimatedCharWidth);
      if (text.length > maxChars && maxChars > 3) {
        return text.substring(0, maxChars - 3) + "...";
      }
      return text;
    }
    $$renderer2.push(`<main class="dashboard svelte-1uha8ag"><div class="header svelte-1uha8ag"><div class="header-toolbar svelte-1uha8ag"><div class="toolbar-icons svelte-1uha8ag"><button type="button" class="toolbar-icon-btn svelte-1uha8ag" title="Ouvrir un projet"${attr("disabled", isLoading, true)}><span class="svelte-1uha8ag">📁</span></button> <button type="button" class="toolbar-icon-btn svelte-1uha8ag" title="Paramètres" disabled><span class="svelte-1uha8ag">⚙️</span></button> <button type="button" class="toolbar-icon-btn svelte-1uha8ag" title="Informations" disabled><span class="svelte-1uha8ag">ℹ️</span></button></div> `);
    if (recentProjects.length > 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<button type="button" class="header-action-btn svelte-1uha8ag"${attr("disabled", isLoading, true)}><span class="action-icon svelte-1uha8ag">⚡</span> <span class="action-text svelte-1uha8ag">Dernier projet</span> <span class="arrow-icon svelte-1uha8ag">→</span></button>`);
    } else {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></div></div> <div class="dashboard-container svelte-1uha8ag" role="region" aria-label="Zone de dépôt de projet">`);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--> <div class="project-selector svelte-1uha8ag"><label class="project-label svelte-1uha8ag" for="project-path">Sélectionnez un projet pour continuer :</label> <div class="project-input-row svelte-1uha8ag"><div class="path-input-container svelte-1uha8ag"><input id="project-path" type="text" placeholder="Chemin du projet..."${attr("value", projectPath)} class="path-input svelte-1uha8ag"/> <div class="input-actions svelte-1uha8ag"><button type="button" class="input-action-btn folder-btn svelte-1uha8ag"${attr("disabled", isLoading, true)} title="Parcourir un dossier"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="svelte-1uha8ag"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" class="svelte-1uha8ag"></path></svg></button> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></div></div> <button type="button" class="continue-btn svelte-1uha8ag"${attr("disabled", !projectPath, true)}>${escape_html("Continuer")}</button></div></div> `);
    if (recentProjects.length > 0) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="recent-projects svelte-1uha8ag"><h2 class="svelte-1uha8ag">Projets récents</h2> <div class="projects-list svelte-1uha8ag">`);
      {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--> <!--[-->`);
      const each_array = ensure_array_like(recentProjects);
      for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
        let project = each_array[$$index];
        $$renderer2.push(`<div class="project-item svelte-1uha8ag"><button type="button" class="project-button svelte-1uha8ag"${attr("disabled", isLoading, true)}><div class="project-info svelte-1uha8ag"><div class="project-header svelte-1uha8ag"><div class="project-name svelte-1uha8ag">${escape_html(truncate(project.name))}</div> <div class="project-date svelte-1uha8ag">${escape_html(formatDate(project.openedAt))}</div></div> <div class="project-path svelte-1uha8ag"${attr("title", project.path)}>${escape_html(truncate(project.path))}</div></div></button></div>`);
      }
      $$renderer2.push(`<!--]--></div></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></div> <footer class="app-footer svelte-1uha8ag"><div class="footer-content svelte-1uha8ag"><div class="footer-section svelte-1uha8ag"><span class="footer-label svelte-1uha8ag">Version:</span> <span class="footer-value svelte-1uha8ag">v0.1.0</span></div> <div class="footer-separator svelte-1uha8ag">|</div> <div class="footer-section svelte-1uha8ag"><span class="footer-label svelte-1uha8ag">Stockage:</span> <span class="footer-value svelte-1uha8ag">Local uniquement</span></div> <div class="footer-separator svelte-1uha8ag">|</div> <div class="footer-section svelte-1uha8ag"><span class="footer-label svelte-1uha8ag">Raccourci:</span> <span class="footer-value svelte-1uha8ag">Glisser-déposer un projet ici.</span></div></div></footer></main> `);
    PageTransition($$renderer2, { show: showTransition });
    $$renderer2.push(`<!---->`);
  });
}
export {
  _page as default
};
