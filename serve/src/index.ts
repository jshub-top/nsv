
import express from "express";
import { join } from "path";
import { sync_node_mirror } from "./lib/node"




sync_node_mirror()

const app = express();

app.use("/dist", express.static(join(__dirname, "./dist")))

app.listen("3000", () => {
    console.log("serve start")
})


