const fs = require("fs");
const axios = require("axios");
const { parse } = require("csv-parse/sync");
const QRCode = require("qrcode");

async function main() {
  const file = process.argv[2];
  const csv = fs.readFileSync("./checkpoints/" + file + ".csv").toString();
  const records = parse(csv, {
    columns: true,
  });
  const apiUrl = "https://passport-spaghett-eth-7fwxk.ondigitalocean.app";
  let i = 1;
  for (const record of records) {
    try {
      const checkpointSlug = record.Slug;
      // Save QR code to file
      if (!fs.existsSync("./checkpoints/qrcodes/" + file)) {
        fs.mkdirSync("./checkpoints/qrcodes/" + file);
      }
      const orderedName = i.toString().padStart(3, "0");
      await QRCode.toFile(
        "./checkpoints/qrcodes/" +
          file +
          "/" +
          orderedName +
          "_" +
          checkpointSlug +
          ".png",
        "https://passport.spaghett-eth.com/collect/" + checkpointSlug
      );
      console.log("DONE");
      i++;
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
