if (process.env.NODE_ENV !== "production") {
  await import("dotenv/config");
}

import express from "express";
import session from "express-session";

import buildConfiguration from "./utils/configuration.util.js";
import isAuthenticated from "./middlewares/isAuthenticated.middleware.js";

const app = express();

const configuration = buildConfiguration();


app.set("loginUrl", configuration.application.loginUrl);

app.use(
  session({
    secret: configuration.application.sessionKey, // A secret to sign the session ID cookie
    resave: false,
    saveUninitialized: true,
    cookie: { maxAge: 60 * 60 * 1000 }, // Example: session lasts 1 hour
  })
);

app.use(isAuthenticated);

app.get("/", (req, res) => {
  res.send("Hello World");
});

const PORT = process.env.PORT || 8080;

app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
