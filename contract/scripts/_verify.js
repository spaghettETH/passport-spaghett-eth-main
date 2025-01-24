const argv = require("minimist")(process.argv.slice(2));
const fs = require("fs");
const child_process = require("child_process");
const { generate, derive } = require("../libs/address_generator");
let configFile;
let scriptFile;

async function run() {
  try {
    configFile = "./configs/" + argv._[0] + ".json";
    const configs = JSON.parse(fs.readFileSync(configFile).toString());
    // Verify SpaghettETH Passport
    let arguments =
      '"SPAGHETTETHPASSPORT"' +
      "," +
      '"SGPP"' +
      "," +
      "'" +
      configs.resolver_address +
      "'";
    fs.writeFileSync(
      "./artifacts/arguments.js",
      `module.exports = [` + arguments + `]`
    );
    child_process.execSync(
      'ETHERSCAN="' +
        configs.etherscan_key +
        '" ' +
        'PROVIDER="' +
        configs.provider +
        '" ' +
        "npx hardhat verify --show-stack-traces --network " +
        configs.network +
        " " +
        configs.passport_address +
        " --constructor-args ./artifacts/arguments.js",
      { stdio: "inherit" }
    );
    console.log("All done, exiting!");
    // Verify SpaghettETH Resolver
    child_process.execSync(
      'ETHERSCAN="' +
        configs.etherscan_key +
        '" ' +
        'PROVIDER="' +
        configs.provider +
        '" ' +
        "npx hardhat verify --show-stack-traces --network " +
        configs.network +
        " " +
        configs.resolver_address,
      { stdio: "inherit" }
    );
    process.exit();
  } catch (e) {
    console.log(e.message);
    process.exit();
  }
}

if (argv._ !== undefined) {
  run();
} else {
  console.log(
    "Can'r run verification, please use script like `yarn verify mumbai`."
  );
}
