import {Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction} from "@solana/web3.js";
import {DRAW_RESULT, LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID} from "../src/constants";
import {claimPrizeIx, getBonusPrizeSeedSigner, getDrawBuffer, setPrizeIx} from "../src";
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
        await sendAndConfirmTransaction(connection, createVaultAtaTx, [prizeAdder]);

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
        await sendAndConfirmTransaction(connection, tx, [prizeAdder]);
        const winnerAta = await createAssociatedTokenAccount(connection, winner, prizeMint, winner.publicKey);
        while (true) {
            const balance = await connection.getBalance(winnerAta);
            if(balance > 0) {
                break;
            }
        }
        const claimIx = claimPrizeIx(winner.publicKey, 4, LOTTERY_ACCOUNT, prizeMint);
        const claimTx = new Transaction().add(claimIx);
        const vaultAtaBefore = await connection.getTokenAccountBalance(vaultAta);
        expect(vaultAtaBefore.value.amount).toBe("1000000000");
        const winnerAtaBefore = await connection.getTokenAccountBalance(winnerAta);
        expect(winnerAtaBefore.value.amount).toBe("0");
        await sendAndConfirmTransaction(connection, claimTx, [winner]);
        const vaultAtaAfter= await connection.getTokenAccountBalance(vaultAta);
        expect(vaultAtaAfter.value.amount).toBe("0");
        const winnerAtaAfter = await connection.getTokenAccountBalance(winnerAta);
        expect(winnerAtaAfter.value.amount).toBe("1000000000");
    }, 120000);
});
