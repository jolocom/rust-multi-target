var addon = require('../native');
import { CryptoUtils, EncryptedWalletUtils } from "@jolocom/vaulted-key-provider"

export const validateEvents = async (
    events: string
): Promise<string> => await addon.validateEvents(events);

export const getIcp = async (): Promise<string> => await addon.getIcp();

export const getIdFromEvent = async (
    event: string
): Promise<string> => await addon.getIdFromEvent(event);

export const walletUtils: EncryptedWalletUtils = {
    newWallet: async (
        id: string,
        pass: string
    ): Promise<string> => await addon.newWallet(id, pass),

    changePass: async (
        encryptedWallet: string,
        id: string,
        oldPass: string,
        newPass: string
    ): Promise<string> => await addon.changePass(
        encryptedWallet,
        id,
        oldPass,
        newPass
    ),

    changeId: async (
        encryptedWallet: string,
        id: string,
        newId: string,
        pass: string
    ): Promise<string> => await addon.changeId(
        encryptedWallet,
        id,
        newId,
        pass
    ),

    newKey: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        keyType: string,
        controller?: string
    ): Promise<string> => await addon.newKey(
        encryptedWallet,
        id,
        pass,
        keyType,
        controller
    ),
    
    getKey: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        key_ref: string
    ): Promise<string> => await addon.getKey(
        encryptedWallet,
        id,
        pass
    ),

    getKeyByController: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        controller: string
    ): Promise<string> => await addon.getKeyByController(
        encryptedWallet,
        id,
        pass,
        controller
    ),
     
    getKeys: async (
        encryptedWallet: string,
        id: string,
        pass: string
    ): Promise<string> => await addon.getKeys(
        encryptedWallet,
        id,
        pass
    ),
    
    sign: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        data: string,
        key_ref: string
    ): Promise<string> => await addon.sign(
        encryptedWallet,
        id,
        pass,
        key_ref,
        data
    ),

    decrypt: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        data: string,
        keyRef: string,
        aad?: string
    ): Promise<string> => await addon.decrypt(
        encryptedWallet,
        id,
        pass,
        keyRef,
        data,
        aad
    )
}

export const cryptoUtils: CryptoUtils = {
    verify: async (
        pkInfo: string,
        data: string,
        signature: string
    ): Promise<boolean> => await addon.verify(
        pkInfo,
        data,
        signature
    ),

    encrypt: async (
        pkInfo: string,
        data: string,
        aad?: string
    ): Promise<string> => await addon.encrypt(
        pkInfo,
        data,
        aad
    ),
    
    getRandom: async (
        len: number
    ): Promise<string> => await addon.getRandom(len),

}
