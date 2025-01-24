const { ethers } = require("ethers");
const fs = require("fs");

async function main() {
  const configs = JSON.parse(fs.readFileSync(process.env.CONFIG).toString());
  const ABI = JSON.parse(
    fs
      .readFileSync(
        "./artifacts/contracts/PassportResolver.sol/PassportResolver.json"
      )
      .toString()
  );
  const provider = new ethers.providers.JsonRpcProvider(configs.provider);
  const contract = new ethers.Contract(
    configs.resolver_address,
    ABI.abi,
    provider
  );

  try {
    // Get the latest block number
    const latestBlock = await provider.getBlockNumber();

    // Fetch SetPassport events from the last 1000 blocks (adjust as needed)
    const events = await contract.queryFilter(
      "PassportSet",
      latestBlock - 1000,
      latestBlock
    );

    console.log(`Found ${events.length} PassportSet events:`);
    events.forEach((event, index) => {
      console.log(`Event ${index + 1}:`);
      console.log(`  Transaction Hash: ${event.transactionHash}`);
      console.log(`  Block Number: ${event.blockNumber}`);
      console.log(`  User: ${event.args.user.toString()}`);
      console.log(`  Name: ${event.args.name}`);
      console.log(`  Description: ${event.args.description}`);
      console.log(`  Image: ${event.args.image}`);
      console.log("---");
    });
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
