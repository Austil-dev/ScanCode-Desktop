import "clsx";
import { w as attr_class } from "../../../chunks/index2.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import "@sveltejs/kit/internal/server";
import "../../../chunks/state.svelte.js";
import { P as PageTransition } from "../../../chunks/PageTransition.js";
import "@tauri-apps/api/core";
function VerticalToolbar($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let clickedItem = null;
    function isActive(path) {
      const currentPath = window.location.pathname;
      if (path === "statistiques") {
        return currentPath === "/stats" || currentPath === "/stats/";
      }
      return currentPath.includes(`/stats/${path}`);
    }
    $$renderer2.push(`<nav class="vertical-toolbar svelte-10ssyv7"><button${attr_class("nav-item svelte-10ssyv7", void 0, { "clicked": clickedItem === "home" })} title="Retour au menu principal"><div class="icon svelte-10ssyv7">🏠</div></button> <div class="separator svelte-10ssyv7"></div> <button${attr_class("nav-item svelte-10ssyv7", void 0, {
      "active": isActive("statistiques"),
      "clicked": clickedItem === "statistiques"
    })} title="Statistiques du projet"><div class="icon svelte-10ssyv7">📈</div></button> <button${attr_class("nav-item svelte-10ssyv7", void 0, {
      "active": isActive("infos"),
      "clicked": clickedItem === "infos"
    })} title="Informations"><div class="icon svelte-10ssyv7">ℹ️</div></button> <button${attr_class("nav-item svelte-10ssyv7", void 0, {
      "active": isActive("donnees"),
      "clicked": clickedItem === "donnees"
    })} title="Données du projet"><div class="icon svelte-10ssyv7">📊</div></button> <button${attr_class("nav-item svelte-10ssyv7", void 0, {
      "active": isActive("structure"),
      "clicked": clickedItem === "structure"
    })} title="Structure du projet"><div class="icon svelte-10ssyv7">🗂️</div></button></nav>`);
  });
}
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    let showTransition = false;
    $$renderer2.push(`<div class="stats-layout svelte-t4rmop">`);
    VerticalToolbar($$renderer2);
    $$renderer2.push(`<!----> <div class="main-content svelte-t4rmop">`);
    children($$renderer2);
    $$renderer2.push(`<!----></div></div> `);
    PageTransition($$renderer2, { show: showTransition });
    $$renderer2.push(`<!---->`);
  });
}
export {
  _layout as default
};
