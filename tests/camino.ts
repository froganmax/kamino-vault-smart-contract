import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Camino } from "../target/types/camino";
import { Connection, Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { createAssociatedTokenAccount, createMint, getAssociatedTokenAddressSync, mintTo } from "@solana/spl-token";
import { connection, payer } from "./config";
import { BN } from "bn.js";

describe("camino", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Camino as Program<Camino>;

  let cbBtcMint;

  it("Simulate cbBtc", async () => {
    console.log("Payer : ", payer.publicKey.toBase58());

    cbBtcMint = await createMint(connection, payer, payer.publicKey, payer.publicKey, 9)
    console.log("cbBtcMint : ", cbBtcMint);

    const payerAta = getAssociatedTokenAddressSync(cbBtcMint, payer.publicKey)

    await createAssociatedTokenAccount(connection, payer, cbBtcMint, payer.publicKey)
    await mintTo(connection, payer, cbBtcMint, payerAta, payer.publicKey, 10 ** 18)
  });

  it("Initialize", async () => {

    // Add your test here.
    const tx = new Transaction()
    const ix = await program.methods
      .initialize()
      .accounts({
        cbBtc: cbBtcMint,
        payer : payer.publicKey
      })
      .signers([payer])
      .instruction()

    tx.add(ix)

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Your transaction signature", sig);
  });

  it("Deposit to cbBTC Pool", async () => {

    // Add your test here.
    const tx = new Transaction()
    const ix = await program.methods
      .depositToCbBtc(new BN(10000))
      .accounts({
        cbBtc: cbBtcMint,
        payer : payer.publicKey,
      })
      .signers([payer])
      .instruction()

    tx.add(ix)

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Your transaction signature", sig);
  });

  it("Deposit cbBTC Pool to Camino Pool", async () => {

    // Add your test here.
    const tx = new Transaction()
    const ix = await program.methods
      .depositCbBtcToCamino(new BN(10000))
      .accounts({
        cbBtc: cbBtcMint,
        payer : payer.publicKey
      })
      .signers([payer])
      .instruction()

    tx.add(ix)

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Your transaction signature", sig);
  });

  it("Withdraw  Camino Pool to cbBTC Pool", async () => {

    // Add your test here.
    const tx = new Transaction()
    const ix = await program.methods
      .withdrawCaminoToCbBtc(new BN(10000))
      .accounts({
        cbBtc: cbBtcMint,
        payer : payer.publicKey
      })
      .signers([payer])
      .instruction()

    tx.add(ix)

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Your transaction signature", sig);
  });


  it("Withdraw  from cbBTC Pool", async () => {

    // Add your test here.
    const tx = new Transaction()
    const ix = await program.methods
      .withdrawFromCbBtc(new BN(10000))
      .accounts({
        cbBtc: cbBtcMint,
        payer : payer.publicKey
      })
      .signers([payer])
      .instruction()

    tx.add(ix)

    tx.feePayer = payer.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash

    console.log(await connection.simulateTransaction(tx));

    const sig = await sendAndConfirmTransaction(connection, tx, [payer])
    console.log("Your transaction signature", sig);
  });
});
