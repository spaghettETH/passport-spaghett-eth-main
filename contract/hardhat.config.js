require("@nomiclabs/hardhat-waffle");
require("@nomiclabs/hardhat-etherscan");

let provider = "http://localhost:8545";
let hardhatConfigs = {
  defaultNetwork: "hardhat",
  networks: {
    hardhat: {},
    amoy: {
      url: provider,
    },
    goerli: {
      url: provider,
    },
    sepolia_op: {
      url: provider,
    },
    optimism: {
      url: provider,
    },
    mainnet: {
      url: provider,
    },
    quadrans: {
      url: provider,
      verify: {
        apiUrl: "https://explorer.quadrans.io/api",
        apiKey: "NO_API_KEY_NEEDED",
        explorerUrl: "https://explorer.quadrans.io/address",
      },
    },
    "base-sepolia": {
      url: provider,
    },
    "base-mainnet": {
      url: provider,
    },
  },
  solidity: "0.8.6",
};

if (process.env.ACCOUNTS !== undefined) {
  for (let k in hardhatConfigs.networks) {
    hardhatConfigs.networks[k].accounts = [];
    for (let a in process.env.ACCOUNTS.split(",")) {
      if (k === "hardhat") {
        hardhatConfigs.networks[k].accounts.push({
          privateKey: process.env.ACCOUNTS.split(",")[a],
          balance: "10000000000000000000000000000000000000",
        });
      } else {
        hardhatConfigs.networks[k].accounts.push(
          process.env.ACCOUNTS.split(",")[a]
        );
      }
    }
  }
}

if (process.env.PROVIDER !== undefined) {
  for (let k in hardhatConfigs.networks) {
    if (k !== "hardhat") {
      hardhatConfigs.networks[k].url = process.env.PROVIDER;
    }
  }
}

console.log("Etherscan key found, adding networks.");
hardhatConfigs.etherscan = {
  apiKey: {
    mainnet: process.env.ETHERSCAN,
    rinkeby: process.env.ETHERSCAN,
    goerli: process.env.ETHERSCAN,
    optimisticEthereum: process.env.ETHERSCAN,
    linea_mainnet: process.env.ETHERSCAN,
    polygon_zkevm: process.env.ETHERSCAN,
    "base-mainnet": process.env.ETHERSCAN,
    "base-sepolia": process.env.ETHERSCAN,
    quadrans: "NO_API_KEY_NEEDED",
  },
  customChains: [
    {
      network: "amoy",
      chainId: 80002,
      urls: {
        apiURL: "https://api-amoy.polygonscan.com/api",
        browserURL: "https://amoy.polygonscan.com/",
      },
    },
    {
      network: "base-sepolia",
      chainId: 84532,
      urls: {
        apiURL: "https://api-sepolia.basescan.org/api",
        browserURL: "https://sepolia.basescan.org",
      },
    },
    {
      network: "base-mainnet",
      chainId: 8453,
      urls: {
        apiURL: "https://api.basescan.org/api",
        browserURL: "https://basescan.org",
      },
    },
    {
      network: "quadrans",
      chainId: 10946,
      urls: {
        apiURL: "https://explorer.quadrans.io/api",
        browserURL: "https://explorer.quadrans.io/address",
      },
    },
    {
      network: "linea_mainnet",
      chainId: 59144,
      urls: {
        apiURL: "https://api.lineascan.build/api",
        browserURL: "https://lineascan.build/",
      },
    },
    {
      network: "polygon_zkevm",
      chainId: 1101,
      urls: {
        apiURL: "https://api-zkevm.polygonscan.com/api",
        browserURL: "https://zkevm.polygonscan.com/",
      },
    },
  ],
};

module.exports = hardhatConfigs;
