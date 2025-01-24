const { ethers, utils } = require("ethers");
const fs = require("fs");
const { generate, derive } = require("../../libs/address_generator");

async function main() {
  const configs = JSON.parse(fs.readFileSync(process.env.CONFIG).toString());
  const ABI = JSON.parse(
    fs
      .readFileSync(
        "./artifacts/contracts/SpaghettETHPassport.sol/SpaghettETHPassport.json"
      )
      .toString()
  );
  const provider = new ethers.providers.JsonRpcProvider(configs.provider);
  let wallet = new ethers.Wallet(configs.owner_key).connect(provider);
  const contract = new ethers.Contract(
    configs.passport_address,
    ABI.abi,
    wallet
  );
  const counter = await contract._checkpointsCounter();
  console.log("Counter: " + counter);
  let ended = false;
  let i = 0;
  while (!ended) {
    try {
      const checkpoint = await contract._checkpoints(i);
      if (checkpoint.name.length === 0) {
        ended = true;
      } else {
        console.log("Checkpoint #" + i + ":", checkpoint);
        console.log("--");
        i++;
      }
    } catch (e) {
      console.log("No tokens left.");
      ended = true;
    }
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
