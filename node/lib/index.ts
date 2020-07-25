var addon = require('../native');

export const validateEvents = async (
    events: string
): Promise<string> => await addon.validateEvents(events);

export const getIcp = async (): Promise<string> => await addon.getIcp();

export const getIdFromEvent = async (
    event: string
): Promise<string> => await addon.getIdFromEvent(event);

export const walletUtils = {
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
        data,
        key_ref
    ),
}

export const cryptoUtils = {
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
