/**
 * Get assets.
 * @param {String} currency - lowercase currency code.
 * @returns {Promise<Array<Object>>} array of assets.
 */
async function get_assets(currency) {
    return await fetch(`http://localhost:5050/get_assets/${currency}`)
        .then((resp) => {
            return resp.json();
        })
        .then((data) => {
            return data;
        })
        .catch((error) => {
            throw error;
        });
}

/**
 * Get PITs.
 * @param {String} currency - lowercase currency code.
 * @returns {Promise<Array<Object>>} array of PITs.
 */
async function get_pits(currency) {
    return await fetch(`http://localhost:5050/get_pits/${currency}`)
        .then((resp) => {
            return resp.json();
        })
        .then((data) => {
            return data;
        })
        .catch((error) => {
            throw error;
        });
}

/**
 * Remove PIT.
 * @param {String} id - id of PIT.
 * @returns {Promise<void>} nothing.
 */
async function remove_pit(id) {
    await fetch(`http://localhost:5050/remove_pit/${id}`)
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