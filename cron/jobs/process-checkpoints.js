const process = require("node:process");
const axios = require("axios");
const dotenv = require("dotenv");
dotenv.config();

async function main() {
  console.log("Processing checkpoints..");
  try {
    const response = await axios.get(
      `${process.env.API_URL}/process-unsigned-checkpoints`
    );
    console.log(response.data);
    process.exit(0);
  } catch (e) {
    console.log(e.message);
    process.exit(1);
  }
}

main();
