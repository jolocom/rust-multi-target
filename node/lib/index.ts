var addon = require('../native');

export const validateEvents = async (events: string): Promise<string> => {
    return await addon.validateEvents(events);
}

export const getIcp = async (): Promise<string> => {
    return await addon.getIcp();
}

export const getIdFromEvent = async (event: string): Promise<string> => {
    return await addon.getIdFromEvent(event);
}

export const newWallet = async (
    id: string,
    pass: string
): Promise<string> => {
    return await addon.newWallet(id, pass);
}

export const changePass = async (
    encryptedWallet: string,
    id: string,
    oldPass: string,
    newPass: string
): Promise<string> => {
    return await addon.changePass(encryptedWallet, id, oldPass, newPass);
}

export const newKey = async (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyType: string,
    controller?: string
): Promise<string> => {
    return await addon.newKey(encryptedWallet, id, pass, keyType, controller)
}

export const getKeys = async (
    encryptedWallet: string,
    id: string,
    pass: string
): Promise<string> => {
    return await addon.getKeys(encryptedWallet, id, pass)
}

export const sign = async (
    encryptedWallet: string,
    id: string,
    pass: string,
    data: string,
    key_ref: string
): Promise<string> => {
    return await addon.sign(encryptedWallet, id, pass, data, key_ref)
}

export const verify = async (
    encryptedWallet: string,
    id: string,
    pass: string,
    data: string,
    key_ref: string,
    signature: string
): Promise<string> => {
    return await addon.verify(encryptedWallet, id, pass, data, key_ref, signature)
}

export const getRandom = async (
    len: number
): Promise<string> => {
    return await addon.getRandom(len)
}
