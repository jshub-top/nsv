const { join } = require("path");
const { exec } = require("shelljs");

require("../dist/init")
exec(`${join(__dirname, "../nsv")} install`)
