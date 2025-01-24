const fs = require("fs");
const axios = require("axios");
const { parse } = require("csv-parse/sync");
const dotenv = require("dotenv");
dotenv.config();

async function main() {
  const file = process.argv[2];
  const csv = fs.readFileSync("./checkpoints/" + file + ".csv").toString();
  const records = parse(csv, {
    columns: true,
  });
  const apiUrl = process.env.API_URL;
  const session = process.env.SESSION;
  for (const record of records) {
    try {
      // Convert date to timestamp
      // https://www.epochconverter.com/
      const timestampStart = record.StartTime;
      const timestampEnd = record.EndTime;
      const eventName = record.EventName;
      const checkpointSlug = record.Slug;
      const checkpointImage = record.Logo;
      const checkpoint =
        record.Speaker + " - " + record.Talk + " " + record.Tags;
      const request = {
        session,
        name: eventName,
        description: checkpoint,
        image: checkpointImage,
        timestamp_start: timestampStart.toString(),
        timestamp_end: timestampEnd.toString(),
        checkpoint_slug: checkpointSlug,
      };
      let action = "INSERT";
      try {
        const checkIfExists = await axios.get(
          apiUrl + "/checkpoint/id/" + checkpointSlug
        );
        if (checkIfExists.data.checkpoint_id !== undefined) {
          console.log(
            "Checkpoint already exists, checking if it needs update..."
          );
          console.log(
            "Check name: " + checkIfExists.data.name,
            checkIfExists.data.name === eventName
          );
          console.log(
            "Check description: " + checkIfExists.data.description,
            checkIfExists.data.description === checkpoint
          );
          console.log(
            "Check image: " + checkIfExists.data.image,
            checkIfExists.data.image === checkpointImage
          );
          console.log(
            "Check timestamp_start: " + checkIfExists.data.timestamp_start,
            checkIfExists.data.timestamp_start === timestampStart.toString()
          );
          console.log(
            "Check timestamp_end: " + checkIfExists.data.timestamp_end,
            timestampEnd,
            checkIfExists.data.timestamp_end === timestampEnd.toString()
          );
          if (
            checkIfExists.data.name !== eventName ||
            checkIfExists.data.description !== checkpoint ||
            checkIfExists.data.image !== checkpointImage ||
            checkIfExists.data.timestamp_start !== timestampStart.toString() ||
            checkIfExists.data.timestamp_end !== timestampEnd.toString()
          ) {
            console.log("Checkpoint needs update!");
            action = "UPDATE";
          } else {
            console.log("Checkpoint is up to date");
            action = "SKIP";
          }
        }
      } catch (e) {
        console.log("FAILED");
        console.log(e.message);
      }
      console.log(request);
      if (action === "INSERT") {
        console.log("Creating checkpoint...");
        const created = await axios.post(
          apiUrl + "/checkpoint/create",
          request
        );
        console.log(created.data);
      } else if (action === "UPDATE") {
        console.log("Updating checkpoint...");
        const updated = await axios.post(
          apiUrl + "/checkpoint/update",
          request
        );
        console.log(updated.data);
      } else if (action === "SKIP") {
        console.log("Checkpoint is up to date, skipping...");
      }
    } catch (e) {
      console.log("FAILED");
      console.log(e.message);
    }
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
