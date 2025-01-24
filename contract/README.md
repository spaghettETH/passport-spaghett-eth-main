# SpaghettETH Passport

## Test in your local network

In order to test your contract in a local network you can run the following command:

```
yarn network
```

This will create a new config file and a new mnemonic in the `configs` folder. Now you can deploy your contract in your local network and test it with provided scripts using folloiwng structure:

```
yarn task <SCRIPT_NAME> <NETWORK>
```