# Solana Calculator

A simple smart contract that implements an on chain calculator.

## Description

This smart contract calculator is to demonstrait how to write a native Solana Program with the methodolgy of seperating the code in the following way, entrypoing, instructions, state, process. The entrypoint is the starting point of the program and passes the instructions and data to the processor. The processor communicates with the instructions module where the instruction and data is deserialized and then passes that data back to the processor where the processor determines which instruction to invoke. Then the processor will communicate with the state module and deserilize any account data, then the processor will update the state data and the data is serialized once again and stored.

The Calculator itself impelemnts all basic arithimtic opperations. The general usage is to first load a operand into the outpout account with a set instruction then followed by an instruction that is either an ADD, SUB, MUL, DIV, or MOD operation with an opperand passed as data that will be applied to the output account. This implementation allows for chaining multiple mathmatical opperations with series of instructions for each opperation.

### Example

SET 4
ADD 4
SUB 2

expected output will be 6

## Getting Started

### Installing

run a local test validator
```
solana-test-validator -r
```

Generate a new wallet
```
solana-keygen new -o ./wallet/id.json --no-bip39-passphrase
```

Airdop sol to wallet
```
solana airdrop 100 --keypair ./wallet/id.json
```

Deploy smart contract to local devnet

```
solana program deploy --keypair ./program/wallet/id.json --output ./program/target/deploy/calc_contract-keypair.json ./program/target/deploy/calc_contract.so
```



### Executing program

Execution Example
```
node ./scripts/index.js --logs --newKeypair --airdrop --compute SET 5 ADD 1 SUB 3
```

### Operators
- SET: set the initial operand
- ADD: addition operator
- SUB: subtraction operator
- MUL: muliply operator
- DIV: division operator
- MOD: modulo operator

general command is `Operator` + `Operand` + `Operator` + `Operand` etc..


## Authors

Contributors names and contact info

Dennis Orbison  
[@Freedom_pk_live](https://twitter.com/Freedom_pk_live)


## License

This project is licensed under the MIT License - see the LICENSE.md file for details