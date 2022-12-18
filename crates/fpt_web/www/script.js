window.onload = () => {
    if (new URL(window.location).searchParams.has("hard_load")) {
        hard_load();
    } else {
        soft_load();
    }
}

async function soft_load() {
    console.time("soft load");

    try {
        await update_values();
        await fill_table();
    }
    catch (error) {
        console.error(error);
    }
    finally {
        console.timeEnd("soft load");
    }
}

async function hard_load() {
    console.time("hard load");

    try {
        await update_assets();
        await update_values();
        await fill_table();
    }
    catch (error) {
        console.error(error);
    }
    finally {
        console.timeEnd("hard load");
    }
}

async function update_assets() {
    await fetch("http://localhost:5050/update_assets")
        .catch((error) => {
            throw error;
        });
}

async function update_values() {
    let currency = document.querySelector("#currency");
    if (currency === null) {
        throw Error("failed finding currency");
    }

    await fetch(`http://localhost:5050/update_values/${currency.value}`)
        .catch((error) => {
            throw error;
        });
}

async function fill_table() {
    let table = document.querySelector("main table tbody");
    let currency = document.querySelector("#currency");
    if (table === null || currency === null) {
        throw Error("failed finding table || failed finding currency");
    }

    await fetch("http://localhost:5050/get_assets")
        .then((resp) => {
            return resp.json();
        }).then((data) => {
            if (typeof data != typeof []) {
                throw Error("unexpected json");
            }

            table.innerHTML = "";

            let seen_categories = [];
            data.sort((a, b) => a.category < b.category);
            data.forEach((asset) => {
                if (!seen_categories.includes(asset.category)) {
                    let rowspan = data.filter((x) => x.category === asset.category).length + 1;
                    table.innerHTML += `<th headers='category' scope='row' rowspan='${rowspan}'>${asset.category}</th>`;
                    seen_categories.push(asset.category);
                }

                let name = asset.name[0].toUpperCase() + asset.name.slice(1).toLowerCase();
                let amount = undefined;
                let code = asset.code;
                let value = undefined;

                switch (asset.category) {
                    case "Cryptocurrency":
                        amount = asset.amount / 100000000;
                        break;
                    default:
                        amount = asset.amount;
                        break;
                }

                switch (currency.value) {
                    case "usd":
                        value = asset.value.toLocaleString("en-US", { style: "currency", currency: "USD" });
                        break;
                    case "eur":
                        value = asset.value.toLocaleString("de-DE", { style: "currency", currency: "EUR" });
                        break;
                    case "nok":
                        value = asset.value.toLocaleString("no-NO", { style: "currency", currency: "NOK" });
                        break;
                    default:
                        throw Error("unsupported currency");
                }

                table.innerHTML += `<tr>
                    <td headers='name'>${name}</td>
                    <td headers='amount'>${amount} ${code}</td>
                    <td headers='value'>${value}</td>
                </tr>`;
            });
        }).catch((error) => {
            throw error;
        });
}