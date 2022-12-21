/**
 * Fill allocation chart.
 * @param {Array<Object>} data - array of assets.
 * @param {String} currency - lowercase currency code.
 * @returns {Promise<void>} nothing.
 */
async function fill_allocation_chart(data, currency) {
    const allocation_chart_el = document.getElementById("#allocation-chart");
    if (allocation_chart_el === null) {
        throw Error("failed finding #allocation-chart");
    }

    const tempObject = {}
    data.forEach((x) => {
        if (tempObject.hasOwnProperty(x.category)) {
            tempObject[x.category] += x.value_in_currency;
        } else {
            tempObject[x.category] = 0;
        }
    });

    const data_values = Object.entries(tempObject).map((x) => x[1]);
    const data_labels = Object.entries(tempObject).map((x) => x[0]);

    const tempChart = Chart.getChart("allocation-chart");
    if (tempChart) {
        tempChart.destroy()
    }

    new Chart(allocation_chart_el, {
        type: "pie",
        data: {
            labels: data_labels,
            datasets: [{
                label: `Value (${currency})`,
                data: data_values,
            }]
        }
    });
}


/**
 * Fill PIT chart.
 * @param {Array<Object>} data - array of PITs.
 * @param {String} currency - lowercase currency code.
 * @returns {Promise<void>} nothing.
 */
async function fill_pit_chart(data, currency) {
    const pit_chart_el = document.querySelector("#pit-chart");
    if (pit_chart_el === null) {
        throw Error("failed finding #pit-chart");
    }

    data.sort((a, b) => a - b);
    const data_values = data.map((x) => x.total_value_in_currency);
    const data_labels = data.map((x) => parse_date(new Date(x.time * 1000)));

    const tempChart = Chart.getChart("pit-chart");
    if (tempChart) {
        tempChart.destroy()
    }

    new Chart(pit_chart_el, {
        type: "line",
        data: {
            labels: data_labels,
            datasets: [{
                label: `Total value (${currency})`,
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
}
