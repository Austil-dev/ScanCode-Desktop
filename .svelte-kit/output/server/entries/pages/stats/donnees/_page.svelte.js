import "clsx";
import "@tauri-apps/api/core";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<main class="page-container svelte-1um2v5v"><div class="header svelte-1um2v5v"><h1 class="svelte-1um2v5v">📊 Données du Projet</h1></div> <div class="content-wrapper svelte-1um2v5v"><div class="content svelte-1um2v5v">`);
    {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div class="loading svelte-1um2v5v"><div class="spinner svelte-1um2v5v"></div> <p class="svelte-1um2v5v">Chargement de l'historique...</p></div>`);
    }
    $$renderer2.push(`<!--]--></div></div></main>`);
  });
}
export {
  _page as default
};
