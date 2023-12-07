const express = require("express");
const { join } = require("path");

const app = express();

app.use("/dist", express.static(join(__dirname, "./dist")))

app.listen("3000", () => {
    console.log("serve start")
})
