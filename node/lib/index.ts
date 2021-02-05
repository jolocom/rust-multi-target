var addon = require('./addon')
import { CryptoUtils, EncryptedWalletUtils } from "@jolocom/vaulted-key-provider"

export const validateEvents = async (
    events: string
): Promise<string> => await addon.validateEvents(events);

export const getIcpFromKeySet = async (config: {
    live_keys: string,
    pre_rotated_keys: string,
    password: string
}): Promise<{
    id: string,
    encryptedWallet: string,
    inceptionEvent: string
}> => JSON.parse(await addon.keriInceptWalletFromKeys(
  config.live_keys,
  config.pre_rotated_keys,
  config.password
));

export const getIcp = async (config: {
    encryptedWallet: string,
    id: string,
    pass: string
}): Promise<{
    id: string,
    encryptedWallet: string,
    inceptionEvent: string
}> => JSON.parse(await addon.keriInceptWallet(
    config.encryptedWallet,
    config.id,
    config.pass
));

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

    addContent: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        content: string
    ): Promise<string> => await addon.addContent(
        encryptedWallet,
        id,
        pass,
        content
    ),

    getKey: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        keyRef: string
    ): Promise<string> => await addon.getKey(
        encryptedWallet,
        id,
        pass,
        keyRef
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

    setKeyController: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        keyRef: string,
        controller: string
    ): Promise<string> => await addon.setKeyController(
        encryptedWallet,
        id,
        pass,
        keyRef,
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
        controller: string,
        data: string
    ): Promise<string> => await addon.sign(
        encryptedWallet,
        id,
        pass,
        controller,
        data
    ),

    decrypt: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        controller: string,
        data: string,
        aad?: string
    ): Promise<string> => await addon.decrypt(
        encryptedWallet,
        id,
        pass,
        controller,
        data,
        aad
    ),

    ecdhKeyAgreement: async (
        encryptedWallet: string,
        id: string,
        pass: string,
        controller: string,
        pubKey: string,
    ): Promise<string> => await addon.ecdhKeyAgreement(
        encryptedWallet,
        id,
        pass,
        controller,
        pubKey
    )
}

export const cryptoUtils: CryptoUtils = {
    verify: async (
        key: string,
        type: string,
        data: string,
        signature: string
    ): Promise<boolean> => await addon.verify(
        key,
        type,
        data,
        signature
    ),

    encrypt: async (
        key: string,
        type: string,
        data: string,
        aad?: string
    ): Promise<string> => await addon.encrypt(
        key,
        type,
        data,
        aad
    ),

    getRandom: async (
        len: number
    ): Promise<string> => await addon.getRandom(len),

}
