import {
  configureChains,
  createConfig,
  disconnect as wagmiDisconnect,
  getAccount,
  getNetwork,
  watchAccount,
  watchNetwork,
} from "@wagmi/core";

import {alchemyProvider} from "@wagmi/core/providers/alchemy";
import {arbitrum, goerli, mainnet, optimism, polygon,} from "@wagmi/core/chains";
import {Web3Modal} from "@web3modal/html";
import {EthereumClient, w3mConnectors,} from "@web3modal/ethereum";
import {defineStore} from "pinia";

export const useWeb3Store = defineStore("web3", {
  state: () => ({
    web3modal: {},
    web3client: {},

    // DEFAULT CHAIN
    defaultChain: import.meta.env.VITE_BLOCKCHAIN,

    walletType: "",
    account: "",
    balance: 0,

    // MEGO WALLET
    email: "",
    megoWalletStep: "",

    // MODAL CONNECTION -> CHOOSE MEGO OR WALLETCONNECT
    isConnectModal: false,
    isClosingConnectModal: false, // only for closing animation
    isExported: false,

    // ALERT MESSAGES
    isAlert: false,
    alertMessage: "",
    network: "",
    apiURL: import.meta.env.VITE_API_URL,

    // ONLY FOR DEBUGGING
    isDebug: import.meta.env.VITE_DEBUG,

    // ONLY FOR UI
    showBigQr: "",
  }),
  actions: {
    // INIT WEB3 WALLET CONNECT FUNCTIONS - (triggered at mounted in App.vue)
    async instantiateWeb3() {
      const app = this;

      // 1. Define constants
      const projectId = import.meta.env.VITE_WALLETCONNECT;
      if (!projectId) {
        throw new Error("You need to provide VITE_WALLETCONNECT env variable");
      }

      const chains = [mainnet, optimism, goerli, polygon, arbitrum];

      // 2. Configure wagmi client
      const { publicClient } = configureChains(chains, [
        alchemyProvider({ apiKey: import.meta.env.VITE_ALCHEMY }),
      ]);
      const wagmiConfig = createConfig({
        autoConnect: true,
        connectors: w3mConnectors({ chains, projectId }),
        publicClient,
      });

      // 3. Create ethereum and modal clients
      const ethereumClient = new EthereumClient(wagmiConfig, chains);
      app.web3client = ethereumClient;
      app.web3modal = new Web3Modal(
        {
          projectId,
        },
        ethereumClient
      );

      console.log("web3modal", app.web3modal);
      // Switch Network AUTO at login
      // if (app.defaultChain === 'mainnet') {
      //   app.web3modal.setDefaultChain(mainnet)
      // } else {
      //   app.web3modal.setDefaultChain(goerli)
      // }

      // Web3Modal check account connection
      console.log("Checking cached connection..");
      app.cachedConnection();
    },

    // CHECK CONNECTION - (triggered at mounted in App.vue)
    async cachedConnection() {
      const app = this;
      app.walletType = await localStorage.getItem("wallet_type");
      app.email = await localStorage.getItem("wallet_email");
      if (app.isDebug) {
        console.log("🚀 init cachedConnection");
        console.log("CACHED_TYPE", app.walletType);
        console.log("CACHED_EMAIL", app.email);
      }

      watchAccount(async (connected) => {
        app.loadAccount(connected);
      });

      watchNetwork(async (connected) => {
        app.loadAccount(connected);
      });

      if (app.walletType === "mego") {
        console.log("WALLET TYPE IS MEGO");
        try {
          if (
            localStorage.getItem("wallet_address") !== null &&
            localStorage.getItem("wallet_address") !== undefined &&
            localStorage.getItem("wallet_address") !== "" &&
            localStorage.getItem("wallet_address") !== "undefined"
          ) {
            app.walletType = await localStorage.getItem("wallet_type");
            app.account = await localStorage.getItem("wallet_address");
            app.email = await localStorage.getItem("wallet_email");
            localStorage.setItem("internalEv", true);
            console.log("ACCOUNT MEGO", app.walletType, app.account, app.email);
          } else {
            localStorage.removeItem("wallet_type");
            localStorage.removeItem("wallet_address");
            localStorage.removeItem("wallet_email");
          }
        } catch (e) {
          console.log("ERROR WHILE GETTING WALLET DATA FROM LOCAL STORAGE");
          console.log(e.message);
          console.log("--------------------");
        }
      } else {
        console.log("🫥 NO ACCOUNT MEGO WALLET CONNECTED");
        app.loadAccount({ isConnected: true });
      }
    },

    async loadAccount(connected) {
      const app = this;
      if (app.isDebug) {
        console.log("🔎 Watch WalletConnect connection", connected);
      }
      if (connected.isConnected) {
        const app = this;
        // trigger provider and fatch address and balance
        const account = app.web3client.getAccount();
        app.account = account.address;
        console.log("account found", app.account);
        app.wallet_type = "metamask";

        // trigger web3 functions
        const { connector } = getAccount();
        if (connector !== undefined) {
          const provider = await connector.getProvider();
          console.log("Provider is:", provider);
          app.network = getNetwork();

          // set connection data on local storage
          if (app?.account) {
            localStorage.setItem("wallet_address", app.account);
            localStorage.setItem("wallet_type", "metamask");
            localStorage.setItem("internalEv", true);
            // Calculate USER balance
            app.walletType = "metamask";
          } else {
            console.log("🫥 NO ACCOUNT WALLETCONNECT CONNECTED");
          }
        }
      } else {
        console.log("NO WALLET CONNECTED");
      }
    },

    // WEB3 WALLETCONNECT TRIGGER FUNCTION
    async connect() {
      const app = this;
      console.log("CONNECTING...");
      app.web3modal.openModal();
      // app.loadState()
    },

    // DISCONNECT ALL ACCOUNT (WALLETCONNECT AND MEGOWALLET)
    async disconnect() {
      const app = this;
      console.log("init web3 logout");
      app.account = "";
      app.walletType = null;
      app.email = null;
      localStorage.setItem("wallet_type", null);
      localStorage.setItem("wallet_address", null);
      localStorage.setItem("wallet_email", null);
      localStorage.setItem("wallet_provider", null);
      await wagmiDisconnect();
    },

    // TRIGGER CHOOSE CONNECTION MODAL
    showConnectModal(isExported = false) {
      const app = this;
      let page = document.getElementsByTagName("html")[0];
      let bodyPage = document.getElementsByTagName("body")[0];
      console.log(app.isConnectModal)
      if (app.isConnectModal === true) {
        page.style.overflowY = "auto";
        bodyPage.style.overflowY = "auto";
        app.isClosingConnectModal = true;
        setTimeout(() => {
          app.isConnectModal = false;
        }, 450);
      } else {
        page.style.overflowY = "hidden";
        bodyPage.style.overflowY = "hidden";
        app.isConnectModal = true;
        app.isClosingConnectModal = false;
      }
      if (isExported) {
        app.isExported = true
      }
    },

    // SHOW/HIDE ALERT MESSAGES
    showAlert(alertMessage) {
      const app = this;
      app.isAlert = !app.isAlert;
      console.log("🚀 ~ isAlert:", alertMessage);
      if (app.isAlert) {
        app.alertMessage = alertMessage;
      } else {
        app.alertMessage = "";
      }
    },
  },
});
