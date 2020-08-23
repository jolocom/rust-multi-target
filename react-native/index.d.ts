import { CryptoUtils, EncryptedWalletUtils } from "@jolocom/vaulted-key-provider";
export declare const validateEvents: (events: string) => Promise<string>;
export declare const getIcp: (config: {
    encryptedWallet: string;
    id: string;
    pass: string;
}) => Promise<{
    id: string;
    encryptedWallet: string;
    inceptionEvent: string;
}>;
export declare const getIdFromEvent: (event: string) => Promise<string>;
export declare const walletUtils: EncryptedWalletUtils;
export declare const cryptoUtils: CryptoUtils;
