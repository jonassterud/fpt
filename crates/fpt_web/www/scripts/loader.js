window.onload = () => {
    load_scripts(
        "scripts/main.js",
        "scripts/common.js",
        "scripts/bindings.js",
        "scripts/charts.js",
        "scripts/vendored/Chart.js.4.1.1.js"
    ).then(() => {
        load(true);
    }).catch((error) => {
        console.error(error);
    });
}

/**
 * Load everything.
 * @param {String} sources - source to a javascript file.
 * @param {bool} async - whether to load script async.
 * @returns {Promise<void>} nothing.
 */
function load_scripts(...sources) {
    let promises = [];

    sources.forEach((source) => {
        promises.push(new Promise((resolve, reject) => {
            const temp = document.createElement("script");
            temp.src = source;
            temp.onload = resolve;
            temp.onerror = reject;
            document.head.appendChild(temp);
        }));
    });

    return Promise.all(promises).catch((error) => console.error(error));
}