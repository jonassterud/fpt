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
            let currency_el = document.querySelector("#currency");
            if (currency_el === null) {
                throw Error("failed finding currency");
            }

            let assets = await get_assets(currency_el.value);
            if (
                assets.length === 0 &&
                await fancy_prompt("No assets were found in the local database.<br>Do you want to remotely load assets?") === true
            ) {
                // console.timeEnd(`${soft ? 'soft' : 'hard'} load`); // doesn't work?
                load(false);
            } else {
                await update_values();
                await fill_table(assets, currency_el.value);
                await save_pit();
                await fill_pit_chart();
                await fill_allocation_chart(assets);
            }
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
 * Fill table with assets.
 * @param {Array<Object>} data - vector of assets.
 * @param {String} currency - lowercase currency code.
 * @returns {Promise<void>} nothing.
 */
async function fill_table(data, currency) {
    let table_el = document.querySelector("main table tbody");
    let total_value_el = document.querySelector("#total_value");
    if (table_el === null || total_value_el === null) {
        throw Error("failed finding table  || failed finding total_value");
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
        let value = parse_currency(asset.value_in_currency, currency);

        // Add asset data to table.
        table_el.innerHTML += `<tr>
                    <td headers='name'>${name}</td>
                    <td headers='amount'>${amount} ${code}</td>
                    <td headers='value'>${value}</td>
                </tr>`;
    });

    // Display total value.
    total_value_el.innerHTML = parse_currency(total_value, currency);
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