import {Connection, Keypair, PublicKey, Transaction} from "@solana/web3.js";
import {DRAW_RESULT, LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID} from "../src/constants";
import {getBonusPrizeSeedSigner, getDrawBuffer, setPrizeIx} from "../src";
import {
    createAssociatedTokenAccount,
    createAssociatedTokenAccountInstruction,
    createMint,
    getAssociatedTokenAddressSync,
    mintTo
} from "@solana/spl-token";
import dotenv from "dotenv";
import fs from 'fs';

dotenv.config();

describe('Bonus Prize', () => {
    it('Should claim a prize', async () => {
        const connection = new Connection('http://localhost:8899');
        const a = process.env.KEY_PATH!;
        const winner = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(process.env.KEY_PATH!, 'utf-8'))));
        const [draw_result,] = PublicKey.findProgramAddressSync([Buffer.from(DRAW_RESULT), getDrawBuffer(4), LOTTERY_ACCOUNT.toBuffer()], NO_LOSS_LOTTERY_ID);
        // Create prize adder
        const prizeAdder = Keypair.generate();
        await connection.requestAirdrop(prizeAdder.publicKey, 1000000000);
        while (true) {
            const balance = await connection.getBalance(prizeAdder.publicKey);
            if(balance > 0) {
                break;
            }
        }
        // Create prize token
        const prizeMint = await createMint(
            connection,
            prizeAdder,
            prizeAdder.publicKey,
            null,
            9);
        const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(4, LOTTERY_ACCOUNT);
        const vaultAta = getAssociatedTokenAddressSync(prizeMint, bonusPrizeSeedSigner, true);
        const vaultAtaIx = await createAssociatedTokenAccountInstruction(
            prizeAdder.publicKey,
            vaultAta,
            bonusPrizeSeedSigner,
            prizeMint);
        const prizeAdderAta = await createAssociatedTokenAccount(connection, prizeAdder, prizeMint, prizeAdder.publicKey);
        const createVaultAtaTx = new Transaction().add(vaultAtaIx);
        await connection.sendTransaction(createVaultAtaTx, [prizeAdder]);

        while (true) {
            const balance = await connection.getBalance(vaultAta);
            if(balance > 0) {
                break;
            }
        }

        await mintTo(connection,
            prizeAdder,
            prizeMint,
            prizeAdderAta,
            prizeAdder,
        1_000_000_000)

        const ix = setPrizeIx(prizeAdder.publicKey, 4, LOTTERY_ACCOUNT, prizeMint, 1_000_000_000);
        const tx = new Transaction().add(ix);
        console.log(await connection.sendTransaction(tx, [prizeAdder]));
        console.log(draw_result.toBase58());
    }, 120000);
});
