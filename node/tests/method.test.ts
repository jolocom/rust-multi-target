import { walletUtils, getIcp, processEvents, resolve, get_kerl } from "../lib";

const db_path = "./test_db"

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
      const prefix = id.split(":")[2]

      // save the event to the DB, and resolve the DID
      await processEvents(inceptionEvent, db_path)
      const ddo = await resolve(prefix, db_path)

      // now do it again, resolved DID doc should be unchanged
      await processEvents(inceptionEvent, db_path)
      const ddoUpdated = await resolve(prefix, db_path)
      
      expect(get_kerl(prefix, db_path)).resolves.toEqual(inceptionEvent)

      return expect(ddoUpdated).toEqual(ddo)
    });
  });
});
