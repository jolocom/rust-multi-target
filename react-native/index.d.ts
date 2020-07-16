declare module 'react-native-substrate-sign' {

  export function keriGetIcp(sk: string, pk: string): Promise<string>;

  export function keriValidateEvents(events: string): Promise<string>;
}
