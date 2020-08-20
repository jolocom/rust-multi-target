declare module 'jolocom-native-utils' {
  export function getIcp(): Promise<string>;
  export function validateEvents(events: string): Promise<string>;
  export function extractIdFromEvent(event: string): Promise<string>;
  export function newWallet: (
    id: string,
    pass: string
  ): Promise<string>;
  
  export function changePass: (
    encryptedWallet: string,
    id: string,
    oldPass: string,
    newPass: string
  ): Promise<string>;
  
  export function changeId: (
    encryptedWallet: string,
    id: string,
    newId: string,
    pass: string
  ): Promise<string>;

  export function newKey: (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyType: string,
    controller?: string
  ): Promise<string>;

  export function addContent: (
    encryptedWallet: string,
    id: string,
    pass: string,
    content: string
  ): Promise<string>;
  
  export function getKey: (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyRef: string
  ): Promise<string>;

  export function getKeyByController: (
    encryptedWallet: string,
    id: string,
    pass: string,
    controller: string
  ): Promise<string>;

  export function setKeyController: (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyRef: string,
    controller: string
  ): Promise<string>;
  
  export function getKeys: (
    encryptedWallet: string,
    id: string,
    pass: string
  ): Promise<string>;

  export function sign: (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyRef: string,
    data: string,
  ): Promise<string>;

  export function decrypt: (
    encryptedWallet: string,
    id: string,
    pass: string,
    keyRef: string,
    data: string,
    aad?: string
  ): Promise<string>;
  
  export function getRandom: (
    len: number
  ): Promise<string>;

  export function verify: (
    key: string,
    type: string,
    data: string,
    sig: string
  ): Promise<boolean>;

  export function encrypt: (
    pkInfo: string,
    toEncrypt: string,
    aad?: string
  ): Promise<string>;
}
