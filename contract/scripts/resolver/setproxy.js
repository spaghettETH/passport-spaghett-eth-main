const { ethers, utils } = require("ethers");
const fs = require("fs");
const { generate, derive } = require("../../libs/address_generator");

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

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
  // console.log(wallet.address)
  let nonce = await provider.getTransactionCount(wallet.address);
  const gasPrice = (await provider.getGasPrice()).mul(2);
  nonce = await provider.getTransactionCount(wallet.address);

  // Adding signer address as proxy address
  const signer = new ethers.Wallet(configs.setter_key).connect(provider);
  const result2 = await contract.setProxyAddress(signer.address, true, {
    nonce: nonce,
    gasPrice,
  });
  console.log("Pending tx:", result2.hash);
  await result2.wait();
  console.log("-> Done!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
