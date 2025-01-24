const Cabin = require("cabin");
const Bree = require("bree");
const express = require("express");

const bree = new Bree({
  logger: new Cabin(),
  jobs: [
    {
      name: "process-checkpoints",
      timeout: false,
      interval: "2m",
    },
    {
      name: "sync-passports",
      timeout: false,
      interval: "30s",
    },
  ],
});

// Create Express app
const app = express();

// Health check route
app.get("/", (req, res) => {
  res.json({ status: "healthy", message: "Cron service is running" });
});

// Start Express server
const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Health check server listening on port ${PORT}`);
});

// start all jobs (this is the equivalent of reloading a crontab):
(async () => {
  await bree.start();
  console.log("Cron jobs started.");
})();
