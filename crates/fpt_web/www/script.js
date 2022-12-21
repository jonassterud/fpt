window.onload = () => {
    load(true);
}

/**
 * Load everything.
 * @param {bool} soft - whether to load assets remotely (*hard load*) or locally (*soft load*).
 * @returns {Promise<void>} nothing.
 */
async function load(soft = true) {
    console.time(`${soft ? 'soft' : 'hard'} load`);

    try {
        if (soft) {
            await update_values();
            await fill_table().then(async (assets) => {
                let message = "No assets were found in the local database.<br>Do you want to remotely load assets?";
                if (assets.length === 0 && await fancy_prompt(message) === true) {
                    // console.timeEnd(`${soft ? 'soft' : 'hard'} load`); // doesn't work?
                    load(false);
                } else {
                    await save_pit();
                    await fill_pit_chart();
                    await fill_allocation_chart(assets);
                }
            });
        } else {
            await update_assets();
            load(true);
        }
    }
    catch (error) {
        console.error(error);
    }
    finally {
        console.timeEnd(`${soft ? 'soft' : 'hard'} load`);
    }
}

/**
 * Update assets.
 * @returns {Promise<void>} nothing.
 */
async function update_assets() {
    await fetch("http://localhost:5050/update_assets")
        .catch((error) => {
            throw error;
        });
}

/**
 * Update asset values.
 * @returns {Promise<void>} nothing.
 */
async function update_values() {
    await fetch("http://localhost:5050/update_values")
        .catch((error) => {
            throw error;
        });
}

/**
 * Save PIT.
 * @returns {Promise<void>} nothing.
 */
async function save_pit() {
    await fetch("http://localhost:5050/save_pit")
        .catch((error) => {
            throw error;
        });
}

/**
 * Fill table with assets.
 * @returns {Promise<Array<Object>>} array of assets.
 */
async function fill_table() {
    let table_el = document.querySelector("main table tbody");
    let total_value_el = document.querySelector("#total_value");
    let currency_el = document.querySelector("#currency");
    if (table_el === null || currency_el === null || total_value_el === null) {
        throw Error("failed finding table || failed finding currency || failed finding total_value");
    }

    let assets_copy = undefined;

    await fetch(`http://localhost:5050/get_assets/${currency_el.value}`)
        .then((resp) => {
            return resp.json();
        })
        .then((data) => {
            if (typeof data != typeof []) {
                throw Error("unexpected json");
            }

            assets_copy = data;

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
        })
        .catch((error) => {
            throw error;
        });

    return assets_copy;
}

/**
 * Parse number as a currency in relevant locale.
 * @param {String} number - number to parse.
 * @param {String} currency - currency code in lowercase.
 * @returns {Promise<String>} parsed currency.
 */
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

/**
 * Parse data in locale.
 * @param {Date} date - date to parse
 * @returns {String} parsed date.
 */
function parse_date(date) {
    return date.toLocaleString();
}

/**
 * Prompt user with a question.
 * @param {String} message - question to ask user.
 * @returns {Promise<bool>} user response.
 */
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

    let button_promise = new Promise((resolve) => {
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

/**
 * Fill PIT chart.
 * @returns {Promise<void>} nothing.
 */
async function fill_pit_chart() {
    let pit_chart_el = document.querySelector("#pit-chart");
    let currency_el = document.querySelector("#currency");
    if (pit_chart_el === null || currency_el === null) {
        throw Error("failed finding pit chart || failed finding currency");
    }

    await fetch(`http://localhost:5050/get_pits/${currency_el.value}`)
        .then((resp) => {
            return resp.json();
        })
        .then((data) => {
            if (typeof data != typeof []) {
                throw Error("unexpected json");
            }

            data.sort((a, b) => a - b);
            let data_values = data.map((x) => x.total_value_in_currency);
            let data_labels = data.map((x) => parse_date(new Date(x.time * 1000)));

            let tmpChart = Chart.getChart("pit-chart");
            if (tmpChart) {
                tmpChart.destroy()
            }

            new Chart(pit_chart_el, {
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
        })
        .catch((error) => {
            throw error;
        })
}

/**
 * Fill allocation chart.
 * @param {Array<Object>} data - array of assets.
 * @returns {Promise<void>} nothing.
 */
async function fill_allocation_chart(data) {
    let allocation_chart_el = document.querySelector("#allocation-chart");
    let currency_el = document.querySelector("#currency");
    if (allocation_chart_el === null || currency_el === null) {
        throw Error("failed finding pit chart || failed finding currency");
    }

    let temp = {}
    data.forEach((x) => {
        if (temp.hasOwnProperty(x.category)) {
            temp[x.category] += x.value_in_currency;
        } else {
            temp[x.category] = 0;
        }
    });

    let data_values = Object.entries(temp).map((x) => x[1]);
    let data_labels = Object.entries(temp).map((x) => x[0]);

    let tmpChart = Chart.getChart("allocation-chart");
    if (tmpChart) {
        tmpChart.destroy()
    }

    new Chart(allocation_chart_el, {
        type: "pie",
        data: {
            labels: data_labels,
            datasets: [{
                label: `Value (${currency_el.value})`,
                data: data_values,
            }]
        }
    });
}