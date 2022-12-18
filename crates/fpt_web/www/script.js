window.onload = () => {
    fill_table();
}

function fill_table() {
    let table = document.querySelector("main table tbody");
    if (table === null) {
        return Error("failed finding table");
    }

    fetch("http://localhost:5050/get_assets")
        .then((resp) => {
            return resp.json();
        }).then((data) => {
            if (typeof data != typeof []) {
                return Error("unexpected json");
            }

            let seen_categories = [];
            data.sort((a, b) => a.category < b.category);
            data.forEach((asset) => {
                if (!seen_categories.includes(asset.category)) {
                    let rowspan = data.filter((x) => x.category === asset.category).length + 1;
                    table.innerHTML += `<th headers='category' scope='row' rowspan='${rowspan}'>${asset.category}</th>`;
                    seen_categories.push(asset.category);
                }

                table.innerHTML += `<tr>
                    <td headers='name'>${asset.name}</td>
                    <td headers='amount'>${asset.amount} ${asset.code}</td>
                    <td headers='value'>${asset.value}</td>
                </tr>`;
            });
        });
}