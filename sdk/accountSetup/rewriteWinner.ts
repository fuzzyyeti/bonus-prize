import fs from 'fs';
import dotenv from 'dotenv'
import {Keypair} from "@solana/web3.js";
dotenv.config();


const drawResult = JSON.parse(fs.readFileSync('./accountSetup/drawResult.json', 'utf8'));

const base64Data: string = drawResult.account.data[0];
const userKeyPath = process.env.KEY_PATH;
const userKey = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(userKeyPath!, 'utf8'))));
const data = Buffer.from(base64Data, 'base64');
// rewrite bytes 8 - 40 with the user key
data.set(userKey.publicKey.toBytes(), 8);
drawResult.account.data = [data.toString('base64'), "base64"];
const newFileWithBadRentEpoch = JSON.stringify(drawResult, null, 2);
const rentEpochPattern = /"rentEpoch":\s*\d+/;
// Number is too big for typescript. It gets converted into a float and the solana-test-validator doesn't line it.
const newFileWithFixedRentEpoch = newFileWithBadRentEpoch.replace(rentEpochPattern, '"rentEpoch": 18446744073709551615');

fs.writeFileSync('./accountSetup/newDrawResult.json', newFileWithFixedRentEpoch);