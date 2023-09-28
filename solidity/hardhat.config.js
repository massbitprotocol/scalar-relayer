require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.19",
  settings: {
    evmVersion: process.env.EVM_VERSION || 'london',
    optimizer: {
        enabled: true,
        runs: 1000,
        details: {
            peephole: true,
            inliner: true,
            jumpdestRemover: true,
            orderLiterals: true,
            deduplicate: true,
            cse: true,
            constantOptimizer: true,
            yul: true,
            yulDetails: {
                stackAllocation: true,
            },
        },
    },
  },
  paths: {
    sources: './contracts',
    artifacts: './artifacts',
},
};
