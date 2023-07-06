import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ed25519Poc } from "../target/types/ed25519_poc";
import * as ed from '@noble/ed25519';
import nacl from "tweetnacl";
import solanaWeb3 from "@solana/web3.js"
import * as assert from 'assert';
import { TransactionInstruction } from '@solana/web3.js';
import { Transaction } from '@solana/web3.js';
import { Connection } from '@solana/web3.js';
import fs from "fs";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
describe("ed25519_poc", () => {
  // Configure the client to use the local cluster.
  //anchor.setProvider(anchor.AnchorProvider.env());
  const connection = new Connection("http://127.0.0.1:8899");
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  const program = anchor.workspace.Ed25519Poc as Program<Ed25519Poc>;
  //let person: anchor.web3.Keypair;
  const MSG = Uint8Array.from(Buffer.from("this is such a good message to sign"));
  let person = anchor.web3.Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync('/home/imentus/imentus_project/ed25519_poc/tests/key.json').toString())));
  let signature: Uint8Array;
  console.log("MSG Bytes", MSG)
  console.log("public key ", person.publicKey)
  it("Is initialized!", async () => {
    let persionbalance = await connection.getBalance(person.publicKey);
    console.log("persionbalance", persionbalance)
    //Ed25519 signature
    signature = nacl.sign.detached(MSG, person.secretKey)
    console.log("signature bytes", signature) // here console signature as a bytes
    let signatureString = Buffer.from(signature).toString('hex') // convert uint8array in string
    console.log("signature String", signatureString) // console signature as a string
    const { blockhash } = await connection.getLatestBlockhash()

    let ix01 = // Ed25519 instruction
      anchor.web3.Ed25519Program.createInstructionWithPublicKey(
        {
          instructionIndex: 0,
          publicKey: person.publicKey.toBytes(), // The public key associated with the instruction (as bytes)
          message: MSG,  // The message to be included in the instruction (as a Buffer)
          signature: signature // The signature associated with the instruction (as a Buffer)
        }
      )
    //let pubkey_tuple = [{name:"pubkey", type:{array:["u8" , 32]}} , {name:"msg" ,  type:"bytes"} , {name:"sig" , }];
    let ix02 = await program.methods.verifyEd25519(
      person.publicKey.toBuffer(),
      Buffer.from(MSG),
      Buffer.from(signature)).accounts({
        sender: person.publicKey,
        ixSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      }).signers([person]).instruction()


    let tx = new anchor.web3.Transaction().add(
      ix01, ix02
    )
    tx.recentBlockhash = blockhash;

    console.log(" Tx Line Number 44 ", tx)

    let result;
    try {
      result = await anchor.web3.sendAndConfirmTransaction(

        connection,
        tx,
        [person]
      );
    } catch (error) {
      assert.fail(`Should not have failed with the following error:\n${error}`);

    }

    console.log("result line number 67 ", result)
  });
});
