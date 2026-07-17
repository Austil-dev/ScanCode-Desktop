export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.png"]),
	mimeTypes: {".png":"image/png"},
	_: {
		client: {start:"_app/immutable/entry/start.Bcld1Iw0.js",app:"_app/immutable/entry/app.CgcNZN-O.js",imports:["_app/immutable/entry/start.Bcld1Iw0.js","_app/immutable/chunks/BzsGREdJ.js","_app/immutable/chunks/BO1RoEkz.js","_app/immutable/chunks/vNg8t8ZU.js","_app/immutable/entry/app.CgcNZN-O.js","_app/immutable/chunks/BO1RoEkz.js","_app/immutable/chunks/rfUfoiIv.js","_app/immutable/chunks/BdmKNva1.js","_app/immutable/chunks/C4qnc7z-.js","_app/immutable/chunks/vNg8t8ZU.js","_app/immutable/chunks/76om7Lfa.js","_app/immutable/chunks/Cg9WxSQy.js","_app/immutable/chunks/7BVUT9eM.js","_app/immutable/chunks/D923GNyH.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js'))
		],
		remotes: {
			
		},
		routes: [
			
		],
		prerendered_routes: new Set(["/","/info","/stats","/stats/donnees","/stats/infos","/stats/structure"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
