import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { Connection, Keypair } from "@solana/web3.js";
import { config } from "dotenv";

config()

const RPC = process.env.RPC || "";
const PAYER = process.env.PAYER || "";

const connection = new Connection(RPC, "processed")
const payer = Keypair.fromSecretKey(bs58.decode(PAYER))

export {
    connection,
    payer
}