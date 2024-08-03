import {Connection, PublicKey, TransactionInstruction} from "@solana/web3.js";
import {BONUS_PRIZE, NO_LOSS_LOTTERY_ID} from "./constants";
import {Buffer} from "buffer";
import {createTransferInstruction, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";


type Prize = {
    prizeMint: PublicKey;
    prizeAmount: number;
}

export async function getAllPrizes(connection: Connection, draw: number, lottery: PublicKey) : Promise<Prize[]> {
    const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(draw, lottery);
    const tokenAccounts = await connection.getParsedTokenAccountsByOwner(bonusPrizeSeedSigner, {programId: TOKEN_PROGRAM_ID});
    const accounts = tokenAccounts.value;
    return accounts.map(account => {
        const prizeMint = new PublicKey(account.account.data.parsed.info.mint);
        const prizeAmount = account.account.data.parsed.info.tokenAmount.uiAmount;
        return {prizeMint, prizeAmount};
    });
}

export function setPrizeIx(payer: PublicKey, draw: number, lottery: PublicKey, prizeMint: PublicKey, amount: number) : TransactionInstruction {
    const payerAta = getAssociatedTokenAddressSync(payer, prizeMint);
    const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(draw, lottery);
    const vaultAta = getAssociatedTokenAddressSync(bonusPrizeSeedSigner, prizeMint);
    return createTransferInstruction(payerAta, vaultAta, payer, amount);
}

export function claimPrizeIx(claimer: PublicKey, draw: number, lottery: PublicKey, prizeMint: PublicKey) : TransactionInstruction {
    console.log("hello");
    const drawBuffer = Buffer.alloc(8);
    drawBuffer.writeBigUInt64LE(BigInt(draw));
    const claimerAta = getAssociatedTokenAddressSync(claimer, prizeMint);

    const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(draw, lottery);
    const vaultAta = getAssociatedTokenAddressSync(bonusPrizeSeedSigner, prizeMint);

    const [drawResultAccount,] = PublicKey.findProgramAddressSync(
        [Buffer.from("draw_result"), drawBuffer, lottery.toBuffer()], new PublicKey(NO_LOSS_LOTTERY_ID));
    return new TransactionInstruction({
        keys: [
            {pubkey: claimer, isWritable: false, isSigner: true},
            {pubkey: bonusPrizeSeedSigner, isWritable: false, isSigner: false},
            {pubkey: claimerAta, isWritable: true, isSigner: false},
            {pubkey: vaultAta, isWritable: true, isSigner: false},
            {pubkey: drawResultAccount, isWritable: false, isSigner: false},
            {pubkey: lottery, isWritable: false, isSigner: false},
            {pubkey: TOKEN_PROGRAM_ID, isWritable: false, isSigner: false}
        ],
        programId: new PublicKey(BONUS_PRIZE),
        data: getDrawBuffer(draw)
    });
}


function getBonusPrizeSeedSigner(draw: number, lottery: PublicKey) : PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from("bonus_prize"), getDrawBuffer(draw), lottery.toBuffer()], new PublicKey(BONUS_PRIZE))[0];
}


function getDrawBuffer(draw: number) : Buffer {
    const drawBuffer = Buffer.alloc(8);
    drawBuffer.writeBigUInt64LE(BigInt(draw));
    return drawBuffer;
}
