const tbody = document.querySelector("#tbody");
const selectors = {};

let info = null;
let busy = false;
let backoff = false;

let retries = 0;

dataLoop();

async function dataLoop() {
    while (true) {
        if (backoff) {
            await new Promise((res) => setTimeout(res, 3000));
            continue;
        }
        if (retries > 4) {
            retries = 0;
            await new Promise((res) => setTimeout(res, 5000));
        }
        try {
            await getData();
        } catch (error) {
            retries++;
        }
        await new Promise((res) => setTimeout(res, 1000));
    }
}

async function getData() {
    try {
        info = await (await fetch(`${root}/data`)).json();
        handleData(info);
    } catch (error) {
        // alert("Error getting data");
    }
}

async function build(project) {
    backoff = true;
    setButtonsDisabledStatus(true);

    try {
        await fetch(`${root}/build`, {
            method: "POST",
            headers: { "Content-Type": "application/x-www-form-urlencoded" },
            body: `project=${project}`,
        });
    } finally {
        setTimeout(() => {
            backoff = false;
        }, 1000);
    }
}

function handleData(data) {
    busy = false;

    var dataArr = [];
    let info;
    for (const key in data) {
        info = data[key];
        info.name = key;
        dataArr[info.order] = info;
    }

    let elem;
    for (const idx in dataArr) {
        elem = dataArr[idx];
        if (elem.status != "IDLE") {
            busy = true;
            setButtonsDisabledStatus(true);
        }
        if (selectors[elem.name]) {
            updateRow(elem);
            continue;
        }

        buildRow(elem);
    }

    if (!busy) {
        setButtonsDisabledStatus(false);
    }
}

function buildRow(data) {
    const key = data.name;
    const row = document.createElement("tr");

    row.id = key;

    tbody.appendChild(row);

    const nameTd = document.createElement("td");
    nameTd.innerHTML = key;
    row.appendChild(nameTd);

    const progressTd = document.createElement("td");
    const progress = document.createElement("progress");
    progress.min = 0;
    progress.max = 100;
    progress.value = data.progress;
    progressTd.appendChild(progress);
    row.appendChild(progressTd);

    const statusTd = document.createElement("td");
    const status = document.createElement("span");
    status.innerHTML = data.status;
    statusTd.appendChild(status);
    row.appendChild(statusTd);

    const lastBuiltTd = document.createElement("td");
    const lastBuilt = document.createElement("span");
    lastBuilt.innerHTML = data.lastBuilt;
    lastBuiltTd.appendChild(lastBuilt);
    row.appendChild(lastBuiltTd);

    const lastBuildDurationTd = document.createElement("td");
    const lastBuildDuration = document.createElement("span");
    lastBuildDuration.innerHTML = data.lastBuildDuration + "s";
    lastBuildDurationTd.appendChild(lastBuildDuration);
    row.appendChild(lastBuildDurationTd);

    const buildTd = document.createElement("td");
    const buildBtn = document.createElement("button");
    buildBtn.innerHTML = "Build";
    buildBtn.addEventListener("click", () => build(key));
    buildTd.appendChild(buildBtn);
    row.appendChild(buildTd);

    selectors[key] = {
        progress: progress,
        status: status,
        lastBuilt: lastBuilt,
        lastBuildDuration: lastBuildDuration,
        buildBtn: buildBtn,
    };
}

function updateRow(data) {
    const key = data.name;
    selectors[key].progress.value = data.progress;
    selectors[key].status.innerHTML = data.status;
    selectors[key].lastBuilt.innerHTML = data.lastBuilt;
    selectors[key].lastBuildDuration.innerHTML = data.lastBuildDuration + "s";
}

function setButtonsDisabledStatus(disabled) {
    for (const key in selectors) {
        if (disabled) {
            selectors[key].buildBtn.setAttribute("disabled", "");
        } else {
            selectors[key].buildBtn.removeAttribute("disabled");
        }
    }
}
