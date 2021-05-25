import { writable, derived } from 'svelte/store';
import * as nearAPI from 'near-api-js';

const { keyStores, connect, WalletConnection } = nearAPI;
const keyStore = new keyStores.BrowserLocalStorageKeyStore();
const config = {
    networkId: process.env.NETWORK_ID,
    keyStore,
    nodeUrl: process.env.NODE_URL,
    walletUrl: process.env.WALLET_URL,
    helperUrl: process.env.HELPER_URL,
    explorerUrl: process.env.EXPLORER_URL 
};

export const near = writable(undefined, async (set) => {
    console.log("Connecting to NEAR");
    const Near = await connect(config);
    set(Near);
});
export const wallet = derived(near, $near => $near && new WalletConnection($near, process.env.APP_NAME));
export const account = derived([wallet, near], async ([$wallet, $near], set) => {
    $wallet && set(await $near.account($wallet.getAccountId() || process.env.ACCOUNT_ID))
});
export const contract = derived(account, $account => {
    return $account && new nearAPI.Contract(
        $account,
        process.env.CONTRACT_ACCOUNT, {
            viewMethods: process.env.VIEW_METHODS.split(' '),
            changeMethods: process.env.CHANGE_METHODS.split(' ')
        }
    )
});