const holodeck_rust = import('./pkg');

holodeck_rust.then(wasm => {
        return wasm.run("localhost");
    })
    .catch(console.error);