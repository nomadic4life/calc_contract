import {
    Keypair,
    Connection,
    clusterApiUrl,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    SYSVAR_RENT_PUBKEY,
} from '@solana/web3.js';

import * as borsh from '@coral-xyz/borsh'

import fs from 'fs/promises';
import util from 'node:util';
import child from 'node:child_process'
const exec = util.promisify(child.exec)

// CONST VALUES::
const SEED = 'output_buffer'
const PROGRAM_KEYPAIR_PATH = './program/target/deploy/calc_contract-keypair.json';

const INIT = 0;
const SET = 1;
const ADD = 2;
const SUB = 3;
const MUL = 4;
const DIV = 5;
const MOD = 6;

const parseVariant = (input) => {

    switch (input) {
        case 'INIT':
            return INIT
        case 'SET':
            return SET
        case 'ADD':
            return ADD
        case 'SUB':
            return SUB
        case 'MUL':
            return MUL
        case 'DIV':
            return DIV
        case 'MOD':
            return MOD
        default:
            return false
    }
}

const connect = async () => {

    if (process.argv.includes("--devnet")) {
        console.log('running on devnet')
        return new Connection(clusterApiUrl('devnet'), { commitment: "confirmed" });
    }

    // await exec('solana-test-validator -r')
    //     .then((e, stdout, stderr) => {
    //         console.log("solana-test-validator running")
    //     })

    console.log('running on localnet')
    return new Connection("http://127.0.0.1:8899", { commitment: "confirmed" });
}

const load = async (conn) => {

    // await exec('solana-test-validator -r')
    //     .then((e, stdout, stderr) => {
    //         console.log("solana-test-validator running")
    //     })

    const keypair = (() => {

        if (process.argv.includes("--newKeypair")) {
            return Keypair.generate();
        }

        // load keypair from file
        return
    })()

    if (process.argv.includes("--airdrop")) {
        const signature = await conn.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL)
        await conn.confirmTransaction(signature)
    }

    if (process.argv.includes("--deploy")) {

        await exec('solana-keygen new -o ./program/wallet/id.json --force --no-bip39-passphrase')
            .then((e, stdout, stderr) => {
                console.log()
                console.log("new", stdout)
            })


        await exec('solana airdrop 100 --keypair ./program/wallet/id.json --commitment finalized')
            .then((e, stdout, stderr) => {
                console.log()
                console.log("airdrop", stdout)
            })

        await exec('solana program deploy ./program/target/deploy/calc_contract.so --keypair ./program/wallet/id.json --commitment finalized')
            .then((e, stdout, stderr) => {
                console.log("program deployed")
            })
    }

    const program = await fs.readFile(PROGRAM_KEYPAIR_PATH, { encoding: "utf8" })
        .then(data => {
            return Keypair.fromSecretKey(Uint8Array.from(JSON.parse(data)))
        })

    conn.onLogs(keypair.publicKey, (logs, ctx) => {

        if (process.argv.includes("--logs")) {
            console.log(logs)
        }

        console.log("\n", logs.logs[logs.logs.length - 3].slice(13), "\n\n")

        process.exit(0)
    }, "confirmed")

    return [program.publicKey, keypair]
}

const state = (program, keypair) => {

    const [key] = PublicKey.findProgramAddressSync(
        [keypair.publicKey.toBuffer(), Buffer.from(SEED)],
        program
    )

    return key
}

const numToByteArray = (num) => {

    let byteArray = new Uint8Array(8);

    for (let index = 0; index < byteArray.length; index++) {
        let byte = Number(num & 255n);
        byteArray[index] = byte;
        num = (num - BigInt(byte)) / 256n;
    }

    return byteArray;
};

const instructionBuffer = (instr) => {

    const variant = Uint8Array.from([instr.variant])
    const operand = numToByteArray(instr.operand)
    const length = variant.length + operand.length

    return Buffer.concat([variant, operand], length)
}

const getTransaction = async (conn, sig) => {

    let transaction = await conn.getParsedTransaction(sig, { maxSupportedTransactionVersion: 0 });

    const date = new Date(transaction.blockTime * 1000);
    const transactionInstructions = transaction.transaction.message.instructions;

    console.log(("-").repeat(20));
    console.log(`Signature: ${transaction.signature}`);
    console.log(`Time: ${date}`);
    console.log(`Status: ${transaction.confirmationStatus}`);
    transactionInstructions.forEach((instruction, n) => {
        // console.log(`---Instructions ${n + 1}: ${instruction.programId.toString()}`);
        console.log(instruction)
    })
    console.log(("-").repeat(20));
}

const execute = async () => {

    const conn = await connect()
    const [program, payer] = await load(conn)
    const stateAccount = state(program, payer)

    const stateAccountData = await conn.getAccountInfo(stateAccount, "finalized")
    const blockhashInfo = await conn.getLatestBlockhash()

    const instr = []
    const tx = new Transaction({ ...blockhashInfo })

    if (!stateAccountData) {

        instr.push(new TransactionInstruction({
            programId: program,
            keys: [
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: stateAccount,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: true,
                },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                }
            ],
            data: instructionBuffer({ variant: 0, operand: 0n }),
        }))
    }

    if (!process.argv.includes("--compute")) {

        // do default execution
        console.log('exit')
        return
    }

    const index = process.argv.lastIndexOf('--compute') + 1
    const input = process.argv.slice(index)

    const iter = input[Symbol.iterator]();

    while (true) {

        const variant = parseVariant(iter.next().value)
        const num = iter.next().value
        const operand = num && BigInt(num)

        if (!variant) {
            break
        }

        instr.push(new TransactionInstruction({
            programId: program,
            keys: [
                {
                    pubkey: payer.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                {
                    pubkey: stateAccount,
                    isSigner: false,
                    isWritable: true,
                }
            ],
            data: instructionBuffer({ variant, operand }),
        }))

    }

    tx.add(...instr)

    tx.sign(payer)



    const txHash = await sendAndConfirmTransaction(
        conn,
        tx,
        [payer]
    );

    console.log()

    conn.confirmTransaction({ ...blockhashInfo, signature: txHash }, { commitment: "confirmed" })

}

execute()