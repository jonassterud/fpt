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