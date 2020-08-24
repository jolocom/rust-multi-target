import { NativeModules } from 'react-native';
import { CryptoUtils, EncryptedWalletUtils } from "@jolocom/vaulted-key-provider"

const { NativeUtils } = NativeModules;

export const validateEvents = async (
    events: string
): Promise<string> => await NativeUtils.validateEvents(events);

export const getIcp = async (config: {
    encryptedWallet: string,
    id: string,
    pass: string
}): Promise<{
    id: string,
    encryptedWallet: string,
    inceptionEvent: string
}> => JSON.parse(await NativeUtils.keriInceptWallet(
    config.encryptedWallet,
    config.id,
    config.pass
));

export const getIdFromEvent = async (
    event: string
): Promise<string> => await NativeUtils.getIdFromEvent(event);

export const walletUtils: EncryptedWalletUtils = {
    newWallet: async (
        id: string,
        pass: string
    ): Promise<string> => await NativeUtils.newWallet(id, pass),

    changePass: async (
        encryptedWallet: string,
        id: string,
        oldPass: string,
        newPass: string
    ): Promise<string> => await NativeUtils.changePass(
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
    ): Promise<string> => await NativeUtils.changeId(
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
    ): Promise<string> => await NativeUtils.newKey(
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
    ): Promise<string> => await NativeUtils.addContent(
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
    ): Promise<string> => await NativeUtils.getKey(
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
    ): Promise<string> => await NativeUtils.getKeyByController(
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
    ): Promise<string> => await NativeUtils.setKeyController(
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
    ): Promise<string> => await NativeUtils.getKeys(
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
    ): Promise<string> => await NativeUtils.sign(
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
    ): Promise<string> => await NativeUtils.decrypt(
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
        key: string,
        type: string,
        data: string,
        signature: string
    ): Promise<boolean> => await NativeUtils.verify(
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
    ): Promise<string> => await NativeUtils.encrypt(
        key,
        type,
        data,
        aad
    ),
    
    getRandom: async (
        len: number
    ): Promise<string> => await NativeUtils.getRandom(len),

}