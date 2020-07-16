var addon = require('../native');

export const validateEvents = async (events: string): Promise<string> => {
    return await addon.validateEvents(events);
}

export const getIcp = async (): Promise<string> => {
    return await addon.getIcp();
}

getIcp().then(async (icp) => {
    const jicp = JSON.parse(icp);
    const [sev, sigs_str] = jicp.icp.split("\r\n\r\n");
    const sigs = sigs_str.split("\n");
    const ev = JSON.parse(sev);
    console.log(ev)
    console.log(sigs)
    const sa = JSON.stringify([jicp.icp])
    console.log(sa)
    console.log(await validateEvents(sa))
})
