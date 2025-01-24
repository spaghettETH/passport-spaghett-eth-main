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
  // console.log(wallet.address)
  let nonce = await provider.getTransactionCount(wallet.address);
  const gasPrice = (await provider.getGasPrice()).mul(2);

  // set proxy address 1
  nonce = await provider.getTransactionCount(wallet.address);
  const result2 = await contract.setProxyAddress(wallet.address, true, {
    nonce: nonce,
    gasPrice,
  });
  console.log("Pending tx:", result2.hash);
  await result2.wait();
  console.log("-> Done!");
  // await sleep(3000)

  // set proxy address 2
  const minter = new ethers.Wallet(configs.minter_key).connect(provider);
  nonce = await provider.getTransactionCount(wallet.address);
  const result3 = await contract.setProxyAddress(minter.address, true, {
    nonce: nonce,
    gasPrice,
  });
  console.log("Pending tx:", result3.hash);
  await result3.wait();

  console.log("-> Done!");
  // await sleep(3000)

  // set proxy address 3
  const signer = new ethers.Wallet(configs.signer_key).connect(provider);
  nonce = await provider.getTransactionCount(wallet.address);
  console.log("Adding minter:", signer.address);
  result4 = await contract.setProxyAddress(signer.address, true, {
    nonce: nonce,
    gasPrice,
  });
  console.log("Pending tx:", result4.hash);
  await result4.wait();
  console.log("-> Done!");
  // await sleep(3000)
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
