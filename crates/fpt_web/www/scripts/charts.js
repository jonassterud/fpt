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

    await get_pits(currency_el.value)
        .then((data) => {
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
        });
}
