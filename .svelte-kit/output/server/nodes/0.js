

export const index = 0;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/layout.svelte.js')).default;
export const universal = {
  "prerender": true,
  "ssr": false
};
export const universal_id = "src/routes/+layout.ts";
export const imports = ["_app/immutable/nodes/0.BnKjaJrm.js","_app/immutable/chunks/C4qnc7z-.js","_app/immutable/chunks/BO1RoEkz.js","_app/immutable/chunks/DjpMgPAa.js","_app/immutable/chunks/Cg9WxSQy.js"];
export const stylesheets = [];
export const fonts = [];
