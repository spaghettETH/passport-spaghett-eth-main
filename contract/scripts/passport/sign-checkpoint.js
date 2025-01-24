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

  const address = configs.owner_address;
  const max = await contract._checkpointsCounter();
  console.log("Max: " + max);
  for (let i = 0; i < max; i++) {
    try {
      const gasPrice = (await provider.getGasPrice()).mul(2);
      const result = await contract.signCheckpoint(i, address, { gasPrice });
      console.log("Waiting at: " + result.hash);
      const receipt = await result.wait();
      console.log("Gas used:", receipt.gasUsed.toString());
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
