/// Determine whether to call hard/soft load based on search params.
window.onload = () => {
    if (new URL(window.location).searchParams.has("hard_load")) {
        hard_load();
    } else {
        soft_load();
    }
}

/// Update asset values and refill table.
async function soft_load() {
    console.time("soft load");

    try {
        await update_values();
        await fill_table();
        await fill_pit_graph();

        if (document.querySelector("table tbody").innerHTML === "") {
            console.timeEnd("soft load");

            let message = "No assets were found in the local database.<br>Do you want to remotely load assets?";
            if (await fancy_prompt(message) === true) {
                return hard_load();
            }
        }
    }
    catch (error) {
        console.error(error);
    }
    finally {
        console.timeEnd("soft load");
    }
}

/// Update assets and then call soft_load by removing search params.
async function hard_load() {
    console.time("hard load");

    try {
        await update_assets();
        window.location.search = "";
    }
    catch (error) {
        console.error(error);
    }
    finally {
        console.timeEnd("hard load");
    }
}

/// Update assets.
async function update_assets() {
    await fetch("http://localhost:5050/update_assets")
        .catch((error) => {
            throw error;
        });
}

/// Update asset values.
async function update_values() {
    await fetch(`http://localhost:5050/update_values`)
        .catch((error) => {
            throw error;
        });

    await fetch(`http://localhost:5050/save_pit`)
        .catch((error) => {
            throw error;
        });
}

/// Fill table with assets.
async function fill_table() {
    let table_el = document.querySelector("main table tbody");
    let total_value_el = document.querySelector("#total_value");
    let currency_el = document.querySelector("#currency");
    if (table_el === null || currency_el === null || total_value_el === null) {
        throw Error("failed finding table || failed finding currency || failed finding total_value");
    }

    await fetch(`http://localhost:5050/get_assets/${currency_el.value}`)
        .then((resp) => {
            return resp.json();
        }).then((data) => {
            if (typeof data != typeof []) {
                throw Error("unexpected json");
            }

            table_el.innerHTML = "";
            let seen_categories = [];
            let total_value = 0;
            data.sort((a, b) => a.category < b.category);

            // Loop trough assets.
            data.forEach((asset) => {
                // Add category header.
                if (!seen_categories.includes(asset.category)) {
                    let rowspan = data.filter((x) => x.category === asset.category).length + 1;
                    table_el.innerHTML += `<th headers='category' scope='row' rowspan='${rowspan}'>${asset.category}</th>`;
                    seen_categories.push(asset.category);
                }

                // Add to total value.
                total_value += asset.value_in_currency;

                // Parse asset data.
                let name = asset.name[0].toUpperCase() + asset.name.slice(1).toLowerCase();
                let amount = asset.amount;
                let code = asset.code;
                let value = parse_currency(asset.value_in_currency, currency_el.value);

                // Add asset data to table.
                table_el.innerHTML += `<tr>
                    <td headers='name'>${name}</td>
                    <td headers='amount'>${amount} ${code}</td>
                    <td headers='value'>${value}</td>
                </tr>`;
            });

            // Display total value.
            total_value_el.innerHTML = parse_currency(total_value, currency_el.value);
        }).catch((error) => {
            throw error;
        });
}

/// Parse currency in locale of currency.
function parse_currency(number, currency) {
    number = Number.parseFloat(number);

    switch (currency) {
        case "usd":
            return number.toLocaleString("en-US", { style: "currency", currency: "USD" });
        case "eur":
            return number.toLocaleString("de-DE", { style: "currency", currency: "EUR" });
        case "nok":
            return number.toLocaleString("no-NO", { style: "currency", currency: "NOK" });
        default:
            throw Error("unsupported currency");
    }
}

/// Parse date in locale.
function parse_date(date) {
    return date.toLocaleString();
}

/// Prompt user (yes or no).
async function fancy_prompt(message) {
    document.body.innerHTML += `
        <div id="fancy-prompt">
            <div>
                <p>${message}</p>
                <div>
                    <button id='yes-button'>Yes</button>
                    <button id='no-button'>No</button>
                </div>
            </div>
        </div>
    `;

    let button_promise = new Promise(function (resolve, _) {
        document.querySelector("#yes-button").addEventListener("click", () => {
            resolve(true);
        })

        document.querySelector("#no-button").addEventListener("click", () => {
            resolve(false);
        });
    });

    let result = await button_promise;
    document.querySelector("#fancy-prompt").remove();

    return result;
}

/// Fill PIT-graph
async function fill_pit_graph() {
    let pit_graph_el = document.querySelector("#pit-graph");
    let currency_el = document.querySelector("#currency");
    if (pit_graph_el === null || currency_el === null) {
        throw Error("failed finding pit graph || failed finding currency");
    }

    await fetch(`http://localhost:5050/get_pits/${currency_el.value}`)
        .then((resp) => {
            return resp.json();
        }).then((data) => {
            if (typeof data != typeof []) {
                throw Error("unexpected json");
            }

            data.sort((a, b) => a - b);
            let data_values = data.map((x) => x.total_value_in_currency);
            let data_labels = data.map((x) => parse_date(new Date(x.time * 1000)));

            let tmpChart = Chart.getChart("pit-graph");
            if (tmpChart) {
                tmpChart.destroy()
            }

            new Chart(pit_graph_el, {
                type: "line",
                data: {
                    labels: data_labels,
                    datasets: [{
                        label: `Total value (${currency_el.value})`,
                        data: data_values,
                        borderWidth: 1
                    }]
                },
                options: {
                    scales: {
                        x: {
                            display: false
                        }
                    }
                }
            });
        });
}