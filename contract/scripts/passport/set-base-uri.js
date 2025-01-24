const { ethers, utils } = require("ethers");
const fs = require('fs');

async function main() {
    const configs = JSON.parse(fs.readFileSync(process.env.CONFIG).toString())
    const ABI = JSON.parse(fs.readFileSync('./artifacts/contracts/SpaghettETHPassport.sol/SpaghettETHPassport.json').toString())
    const provider = new ethers.providers.JsonRpcProvider(configs.provider);
    let wallet = new ethers.Wallet(configs.owner_key).connect(provider)
    const contract = new ethers.Contract(configs.passport_address, ABI.abi, wallet)

    const baseUri = "https://ipfs.io/ipfs/"
    try {
        const gasPrice = (await provider.getGasPrice()).mul(2)
        const result = await contract.setBaseUri(baseUri, { gasPrice })
    } catch (e) {
        console.log("FAILED")
        console.log(e.message)
    }

}

main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
