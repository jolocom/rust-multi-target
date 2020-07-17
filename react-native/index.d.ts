declare module 'jolocom-native-utils' {
  export function getIcp(): Promise<string>;
  export function validateEvents(events: string): Promise<string>;
  export function extractIdFromEvent(event: string): Promise<string>;
}
