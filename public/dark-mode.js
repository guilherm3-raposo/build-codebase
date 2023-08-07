const checkbox = document.querySelector("#toggleDarkMode");

let config;
try {
    config = JSON.parse(localStorage.getItem("buildTool"));
    if (!config) {
        throw new Error();
    }
} catch (error) {
    config = { darkMode: false };
}

checkbox.addEventListener("change", () => {
    config.darkMode = !config.darkMode;
    setDarkMode(config.darkMode);
});

if (config.darkMode) {
    checkbox.checked = true;
    setDarkMode(true);
}

function setDarkMode(dark) {
    if (dark) {
        document.documentElement.classList.add("dark");
    } else {
        document.documentElement.classList.remove("dark");
    }
    localStorage.setItem("buildTool", JSON.stringify(config));
}

setTimeout(() => {
    document.documentElement.classList.add("transitions");
}, 1000);
