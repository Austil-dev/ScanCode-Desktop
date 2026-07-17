import "clsx";
import { o as onDestroy } from "../../../chunks/index-server.js";
import "chart.js/auto";
import "@tauri-apps/api/core";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    onDestroy(() => {
    });
    $$renderer2.push(`<main class="stats-container svelte-16pwk6k"><div class="header svelte-16pwk6k"><h1 class="svelte-16pwk6k">Statistiques</h1></div> <div class="content-wrapper svelte-16pwk6k">`);
    {
      $$renderer2.push("<!--[!-->");
      {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]-->`);
    }
    $$renderer2.push(`<!--]--></div></main>`);
  });
}
export {
  _page as default
};
