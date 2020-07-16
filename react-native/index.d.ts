declare module 'jolocom-native-utils' {
  export function keriGetIcp(): Promise<string>;
  export function keriValidateEvents(events: string): Promise<string>;
}
