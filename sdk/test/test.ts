import {PublicKey} from "@solana/web3.js";
import {DRAW_RESULT, LOTTERY_ACCOUNT, NO_LOSS_LOTTERY_ID} from "../src/constants";
import {getDrawBuffer} from "../src";

describe('Sample Test', () => {
    it('should return true', () => {
        const [draw_result,] = PublicKey.findProgramAddressSync([Buffer.from(DRAW_RESULT), getDrawBuffer(4), LOTTERY_ACCOUNT.toBuffer()], NO_LOSS_LOTTERY_ID);
        console.log(draw_result.toBase58());
    });
});
