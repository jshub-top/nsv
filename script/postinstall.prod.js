const { join } = require("path");
const { exec } = require("shelljs");
const { existsSync } = require("fs-extra");

existsSync(join(__dirname, "../src")) && exec("npm run build && exit 0")
