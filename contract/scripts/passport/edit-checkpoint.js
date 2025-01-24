const { ethers, utils } = require("ethers");
const fs = require("fs");
const randomSentence = require("random-sentence");

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

  const checkpoint = randomSentence({ words: 8 });
  try {
    const timestampStart = 0;
    const timestampEnd = 99999999999999;
    const eventSlug = "SPAGHETTETH_2024";
    const gasPrice = (await provider.getGasPrice()).mul(2);
    const checkpointId = 1;
    const result = await contract.addCheckpoint(
      checkpoint,
      eventSlug,
      timestampStart,
      timestampEnd,
      checkpointId,
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

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
