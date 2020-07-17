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
