import { walletUtils, getIcp, processEvents, resolve } from "../lib";

const db_path = "../db.db"

describe("Local DID Resolver", () => {
  describe("getResolver", () => {
    it("It should fail to resolve an unknown local DID", async () => {
      const testDid = 'did:jun:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA'

      return expect(resolve(testDid.split(":")[2], db_path)).rejects.toBeTruthy()
    });

    it('It should correctly register a known local DID', async () => {
      let idNone = "none"
      let pass = "pass"
      let encryptedWallet = await walletUtils.newWallet(idNone, pass)
      const icp_data = await getIcp({encryptedWallet, id: idNone, pass})
      const { inceptionEvent, id } = icp_data

      // save the event to the DB, and resolve the DID
      await processEvents(inceptionEvent, db_path)
      console.log(id)
      const ddo = await resolve(id, db_path)

      // now do it again, resolved DID doc should be unchanged
      await processEvents(inceptionEvent, db_path)
      const ddoUpdated = await resolve(id, db_path)

      return expect(ddoUpdated).toEqual(ddo)
    });
  });
});
