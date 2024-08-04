import {Connection, PublicKey, TransactionInstruction} from "@solana/web3.js";
import {BONUS_PRIZE, BONUS_PRIZE_ID, DRAW_RESULT, NO_LOSS_LOTTERY_ID} from "./constants";
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
    const drawBuffer = getDrawBuffer(draw);
    const claimerAta = getAssociatedTokenAddressSync(claimer, prizeMint);

    const bonusPrizeSeedSigner = getBonusPrizeSeedSigner(draw, lottery);
    const vaultAta = getAssociatedTokenAddressSync(bonusPrizeSeedSigner, prizeMint);

    const [drawResultAccount,] = PublicKey.findProgramAddressSync(
        [Buffer.from(DRAW_RESULT), drawBuffer, lottery.toBuffer()], new PublicKey(NO_LOSS_LOTTERY_ID));
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
        programId: BONUS_PRIZE_ID,
        data: getDrawBuffer(draw)
    });
}


function getBonusPrizeSeedSigner(draw: number, lottery: PublicKey) : PublicKey {
    return PublicKey.findProgramAddressSync(
        [Buffer.from(BONUS_PRIZE), getDrawBuffer(draw), lottery.toBuffer()], new PublicKey(BONUS_PRIZE))[0];
}


export function getDrawBuffer(draw: number) : Buffer {
    const drawBuffer = Buffer.alloc(8);
    drawBuffer.writeBigUInt64LE(BigInt(draw),0);
    return drawBuffer;
}
