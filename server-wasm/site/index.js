const js = import("./node_modules/@andyjsbell/server-wasm/server_wasm.js");
js.then(js => {
    js.run_server();    
});