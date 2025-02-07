import { Hono } from "@hono/hono";
import { serveStatic } from "@hono/hono/deno";
import { ccourt_enbank, scourt_enbank, scourt_bulletin } from "./route/main.ts";

const app = new Hono();

app.use("/favicon.ico", serveStatic({ path: "./static/favicon.ico" }));

app.get("/", (c) => c.text("Hono!"));
app.get("/헌법재판소/판례공보", ccourt_enbank);
app.get("/대법원/전원합의체", scourt_enbank);
app.get("/대법원/판례공보", scourt_bulletin);

Deno.serve(app.fetch);
