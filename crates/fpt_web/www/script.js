window.onload = () => {
    fill_table();
}

function fill_table() {
    let table = document.querySelector("main table");
    if (table === null) {
        return Error("failed finding table");
    }

    // TODO: Get assets
}