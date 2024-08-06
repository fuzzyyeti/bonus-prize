import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import {
  LOTTERY_ACCOUNT,
} from "../src/constants";
import {
  claimPrizeIx,
  getAllPrizes,
  getBonusPrizeSeedSigner,
  setPrizeIx,
} from "../src";
import {
  createAssociatedTokenAccount,
  createAssociatedTokenAccountInstruction,
  createMint, createMintToInstruction,
  getAssociatedTokenAddressSync,
  mintTo,
} from "@solana/spl-token";
import dotenv from "dotenv";
import fs from "fs";

dotenv.config();

const connection = new Connection("http://127.0.0.1:8899");
const winner = Keypair.fromSecretKey(
  new Uint8Array(JSON.parse(fs.readFileSync(process.env.KEY_PATH!, "utf-8")))
);
let prizeAdder: Keypair;
prizeAdder = Keypair.generate();

describe("Test bonus prize program SDK", () => {
  beforeAll(async () => {
    const airdropSig = await connection.requestAirdrop(
      prizeAdder.publicKey,
      1000000000
    );
    await connection.confirmTransaction(airdropSig);
  }, 10000000);
  it("Sets and claims a prize", async () => {
    const { prizeMint, vaultAta } = await createPrizeToken(
      connection,
      4,
      prizeAdder
    );
    const ix = setPrizeIx(
      prizeAdder.publicKey,
      4,
      LOTTERY_ACCOUNT,
      prizeMint,
      1_000_000_000
    );
    const tx = new Transaction().add(ix);
    await sendAndConfirmTransaction(connection, tx, [prizeAdder], {
      commitment: "finalized",
    });
    const winnerAta = await createAssociatedTokenAccount(
      connection,
      winner,
      prizeMint,
      winner.publicKey
    );
    while (true) {
      const balance = await connection.getBalance(winnerAta);
      if (balance > 0) {
        break;
      }
    }
    const vaultAtaBefore = await connection.getTokenAccountBalance(vaultAta);
    expect(vaultAtaBefore.value.amount).toBe("1000000000");
    const winnerAtaBefore = await connection.getTokenAccountBalance(winnerAta);
    expect(winnerAtaBefore.value.amount).toBe("0");

    const claimIx = claimPrizeIx(
      winner.publicKey,
      4,
      LOTTERY_ACCOUNT,
      prizeMint
    );
    const claimTx = new Transaction().add(claimIx);
    await sendAndConfirmTransaction(connection, claimTx, [winner], {
      commitment: "finalized",
    });
    const vaultAtaAfter = await connection.getTokenAccountBalance(vaultAta);
    expect(vaultAtaAfter.value.amount).toBe("0");
    const winnerAtaAfter = await connection.getTokenAccountBalance(winnerAta);
    expect(winnerAtaAfter.value.amount).toBe("1000000000");
  }, 200000);
  it("Finds all prizes for a draw", async () => {
    while (true) {
      const balance = await connection.getBalance(prizeAdder.publicKey);
      if (balance > 0) {
        break;
      }
    }
    //Generate a new draw so it doesn't add more ATAs to last times.
    const draw = Math.floor(Math.random() * 1000000);
    const prize1 = await createPrizeToken(connection, draw, prizeAdder);
    const prize2 = await createPrizeToken(connection, draw, prizeAdder);
    const prize3 = await createPrizeToken(connection, draw, prizeAdder);
    const allPrizes = await getAllPrizes(connection, draw, LOTTERY_ACCOUNT);
    expect(allPrizes.length).toBe(3);
    expect(allPrizes.every((prize) => prize.prizeAmount === 0)).toBe(true);
    expect(allPrizes.map((prize) => prize.prizeMint.toBase58())).toContain(
      prize1.prizeMint.toBase58()
    );
    expect(allPrizes.map((prize) => prize.prizeMint.toBase58())).toContain(
      prize2.prizeMint.toBase58()
    );
    expect(allPrizes.map((prize) => prize.prizeMint.toBase58())).toContain(
      prize3.prizeMint.toBase58()
    );
  }, 200000);
});

async function createPrizeToken(
  connection: Connection,
  draw: number,
  prizeAdder: Keypair
) {
  const prizeMint = await createMint(
    connection,
    prizeAdder,
    prizeAdder.publicKey,
    null,
    9
  );
  const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(draw, LOTTERY_ACCOUNT);
  const vaultAta = getAssociatedTokenAddressSync(
    prizeMint,
    bonusPrizeSeedSigner,
    true
  );
  const prizeAdderAta = getAssociatedTokenAddressSync(
      prizeMint,
        prizeAdder.publicKey,
  );
  const vaultAtaIx = await createAssociatedTokenAccountInstruction(
    prizeAdder.publicKey,
    vaultAta,
    bonusPrizeSeedSigner,
    prizeMint
  );
  const prizeAdderIx = await createAssociatedTokenAccountInstruction(
    prizeAdder.publicKey,
    prizeAdderAta,
    prizeAdder.publicKey,
    prizeMint
  );
  const mintToIx = await createMintToInstruction(
    prizeMint,
    prizeAdderAta,
    prizeAdder.publicKey,
    1_000_000_000
  );
  const createVaultAtaTx = new Transaction().add(vaultAtaIx).add(prizeAdderIx).add(mintToIx);
  await sendAndConfirmTransaction(connection, createVaultAtaTx, [prizeAdder], {
    commitment: "finalized",
  });
  return { prizeMint, vaultAta };
}
