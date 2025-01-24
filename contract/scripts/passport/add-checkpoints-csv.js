const { ethers, utils } = require("ethers");
const fs = require("fs");

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
  const csv = fs.readFileSync("./checkpoints/napuleth.csv").toString();
  let i = 1;
  for (const line of csv.split("\n")) {
    try {
      const [speaker, talk, tags, logo] = line.split(",");
      const timestampStart = 0;
      const timestampEnd = 99999999999999;
      const eventName = "NapulETH 2024";
      const checkpointSlug = "NAPUL_ETH_2024_TALK_" + i;
      const checkpointImage = logo;
      const checkpoint = speaker + " - " + talk + " " + tags;
      const gasPrice = (await provider.getGasPrice()).mul(2);
      i++;
      console.log("Adding checkpoint: " + checkpoint);
      const result = await contract.addCheckpoint(
        eventName,
        checkpoint,
        timestampStart,
        timestampEnd,
        checkpointSlug,
        checkpointImage,
        { gasPrice }
      );
      console.log("Waiting at: " + result.hash);
      const receipt = await result.wait();
      console.log(receipt);
      console.log("Gas used:", receipt.gasUsed.toString());
      const counter = await contract._checkpointsCounter();
      console.log("Checkpoint counter is:", counter.toString());
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
