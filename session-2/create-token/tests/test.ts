import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import type { CreateToken } from "../target/types/create_token";

describe("Create Tokens", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CreateToken as anchor.Program<CreateToken>;

  const metadata = {
    name: "Solana Workshop",
    symbol: "WORKSOL",
    uri: "https://vidofcggcqpgzeenvbdn.supabase.co/storage/v1/object/sign/solana/create-token.json?token=eyJraWQiOiJzdG9yYWdlLXVybC1zaWduaW5nLWtleV82ZDc3YTE3OS03ODAxLTRlNWQtYmY2MS1lMDI0OTFkYWNiNTkiLCJhbGciOiJIUzI1NiJ9.eyJ1cmwiOiJzb2xhbmEvY3JlYXRlLXRva2VuLmpzb24iLCJpYXQiOjE3NzIxMDcxNTcsImV4cCI6MTc3NDY5OTE1N30.nSkKburntaToWM1q6kgoAckrkg6ksuzs3KRJsd4dr3A",
  };

  it("Create an SPL Token!", async () => {
    // Generate new keypair to use as address for mint account.
    const mintKeypair = new Keypair();

    // SPL Token default = 9 decimals
    const transactionSignature = await program.methods
      .createTokenMint(9, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });

  it("Create an NFT!", async () => {
    // Generate new keypair to use as address for mint account.
    const mintKeypair = new Keypair();

    // NFT default = 0 decimals
    const transactionSignature = await program.methods
      .createTokenMint(0, metadata.name, metadata.symbol, metadata.uri)
      .accounts({
        payer: payer.publicKey,
        mintAccount: mintKeypair.publicKey,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("Success!");
    console.log(`   Mint Address: ${mintKeypair.publicKey}`);
    console.log(`   Transaction Signature: ${transactionSignature}`);
  });
});
