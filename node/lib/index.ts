var addon = require('../native');

export const enum KeyTypes = {
    jwsVerificationKey2020 = "JwsVerificationKey2020",
    ecdsaSecp256k1VerificationKey2019 = "EcdsaSecp256k1VerificationKey2019",
    ed25519VerificationKey2018 = "Ed25519VerificationKey2018",
    gpgVerificationKey2020 = "GpgVerificationKey2020",
    rsaVerificationKey2018 = "RsaVerificationKey2018",
    x25519KeyAgreementKey2019 = "X25519KeyAgreementKey2019",
    schnorrSecp256k1VerificationKey2019 = "SchnorrSecp256k1VerificationKey2019",
    ecdsaSecp256k1RecoveryMethod2020 = "EcdsaSecp256k1RecoveryMethod2020",
}

export const validateEvents = async (events: string): Promise<string> => {
    return await addon.validateEvents(events);
}

export const getIcp = async (): Promise<string> => {
    return await addon.getIcp();
}

export const getIdFromEvent = async (event: string): Promise<string> => {
    return await addon.getIdFromEvent(event);
}

export const newWallet = async (id: string, pass: string): Promise<string> => {
    return await addon.newWallet(id, pass);
}

export const changePass = async (encryptedWallet: string, id: string, oldPass: string,
                                newPass: string): Promise<string> => {
    return await addon.changePass(encryptedWallet, id, oldPass, newPass);
}

export const newKey = async (encryptedWallet: string, id: string, pass: string, keyType: KeyTypes): Promise<string> => {
    return await addon.newKey(encryptedWallet, id, pass, keyType)
}

export const getKeys = async (encryptedWallet: string, id: string, pass: string): Promise<string> => {
    return await addon.getKeys(encryptedWallet, id, pass)
}

export const sign = async (encryptedWallet: string, id: string, pass: string, data: string, key_ref: string): Promise<string> => {
    return await addon.sign(encryptedWallet, id, pass, data, key_ref)
}

export const verify = async (encryptedWallet: string, id: string, pass: string, data: string, key_ref: string, signature: string): Promise<string> => {
    return await addon.verify(encryptedWallet, id, pass, data, key_ref, signature)
}
