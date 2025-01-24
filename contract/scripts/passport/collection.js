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
  const totalSupply = await contract.totalSupply();
  console.log("TOTAL SUPPLY IS: " + totalSupply);
  let ended = false;
  let i = 1;
  let errors = 0;
  const baseUri = await contract._baseTokenURI();
  console.log("BASE URI IS: " + baseUri);
  while (!ended) {
    try {
      const owner = await contract.ownerOf(i);
      const uri = await contract.tokenURI(i, { gasLimit: "800000000000" });
      console.log("TOKENID: " + i, "OWNER IS", owner);
      if (baseUri.length === 0) {
        console.log(Buffer.from(uri.split("base64,")[1], "base64").toString());
        const decodedStr = JSON.parse(
          Buffer.from(uri.split("base64,")[1], "base64").toString()
        );
        console.log(decodedStr);
        console.log("--");
      } else {
        console.log("TOKEN URI IS: " + uri);
      }
      i++;
      errors = 0;
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
