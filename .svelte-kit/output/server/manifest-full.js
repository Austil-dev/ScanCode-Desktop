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
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/2.js')),
			__memo(() => import('./nodes/3.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js')),
			__memo(() => import('./nodes/6.js')),
			__memo(() => import('./nodes/7.js')),
			__memo(() => import('./nodes/8.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/",
				pattern: /^\/$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			},
			{
				id: "/info",
				pattern: /^\/info\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 4 },
				endpoint: null
			},
			{
				id: "/stats",
				pattern: /^\/stats\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 5 },
				endpoint: null
			},
			{
				id: "/stats/donnees",
				pattern: /^\/stats\/donnees\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 6 },
				endpoint: null
			},
			{
				id: "/stats/infos",
				pattern: /^\/stats\/infos\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 7 },
				endpoint: null
			},
			{
				id: "/stats/structure",
				pattern: /^\/stats\/structure\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 8 },
				endpoint: null
			}
		],
		prerendered_routes: new Set([]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
