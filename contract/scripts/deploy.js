const hre = require("hardhat");
const fs = require("fs");

async function main() {
  const configs = JSON.parse(fs.readFileSync(process.env.CONFIG).toString());
  console.log("Deploying contracts..");

  // Deploying resolver
  const Resolver = await hre.ethers.getContractFactory("PassportResolver", {});
  const resolver = await Resolver.deploy();
  await resolver.deployed();
  console.log("Resolver deployed to:", resolver.address);
  configs.resolver_address = resolver.address;

  // Deploying passport
  const Passport = await hre.ethers.getContractFactory(
    "SpaghettETHPassport",
    {}
  );
  const passport = await Passport.deploy(
    "SPAGHETTETHPASSPORT",
    "SGPP",
    resolver.address
  );
  console.log("Deploy transaction is: " + passport.deployTransaction.hash);
  await passport.deployed();
  console.log("Contract deployed to:", passport.address);
  configs.passport_address = passport.address;

  fs.writeFileSync(process.env.CONFIG, JSON.stringify(configs, null, 4));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
