const { ethers, utils } = require("ethers");
const fs = require("fs");
const { generate, derive } = require("../../libs/address_generator");

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
  let wallet = new ethers.Wallet(configs.owner_key).connect(provider);
  const contract = new ethers.Contract(
    configs.resolver_address,
    ABI.abi,
    wallet
  );

  const image =
    "https://ipfs.io/ipfs/QmPWVUeDTSie3urizvkXLqYxUxa7wggZ24k8QeQCxbfeb2";
  const name = "turinglabs";
  const description =
    "Polyhedric Blockchain Developer + " + new Date().toISOString();

  try {
    const gasPrice = (await provider.getGasPrice()).mul(2);
    const result = await contract.setProfile(
      name,
      description,
      image,
      configs.owner_address,
      { gasPrice }
    );
    console.log("Waiting at: " + result.hash);
    await result.wait();
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
