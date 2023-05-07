const { join } = require("path");
const { exec } = require("shelljs");
const { existsSync } = require("fs-extra");


existsSync(join(__dirname, "../dist/init.js")) && require("../dist/init")
console.log("999")
// 如果是在 github ci 中或者 nsv install 的
if (process.env["NSV_STATUS"] === 2) process.exit(0)
exec(`${join(__dirname, "../nsv")} install`)
