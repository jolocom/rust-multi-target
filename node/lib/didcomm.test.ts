let wallet = require('./addon')

interface DidcommHeader {
    id: number;
    type: string;
    to?: string[];
    from?: string;
    createdTime?: number;
    expiresTime?: number;
    fromPrior?: string;
}

describe('didcomm', function() {
    it('create', async function() {
        let message = await wallet.createDidcommMessage();
        expect(message.length).not.toBe(0);
    });

    it('seal', async function() {
        let message = await wallet.createDidcommMessage();
        let h = new_header(message);
        let s_header = JSON.stringify(h);
        let ew = JSON.parse(await new_wallet());
        let sealed = await new_sealed_message(
            message,
            s_header,
            ew.newEncryptedState,
            "did:xyz:ulapcuhsatnpuhza930hpu34n",
            "superbpass"
            );
        let jwe = JSON.parse(sealed);
        expect(jwe).toHaveProperty("from");
        expect(jwe).toMatchObject({from: "ulapcuhsatnpuhza930hpu34n"});
    });

    it('receive', async function() {
        let message = await wallet.createDidcommMessage();
        let h = new_header(message);
        let ew = await new_wallet();
        let s_header = JSON.stringify(h);
        let sealed = await new_sealed_message(message, s_header, ew, "did:xyz:ulapcuhsatnpuhza930hpu34n", "superbpass");
        let received = await wallet.receiveDidcommMessage(
            ew,
            "did:xyz:ulapcuhsatnpuhza930hpu34n",
            "superbpass",
            "ulapcuhsatnpuhza930hpu34n",
            sealed,
            s_header,
            null,
        );
        let raw = JSON.parse(received);
        expect(raw).toHaveProperty("body");
        expect(raw).toMatchObject({from: "ulapcuhsatnpuhza930hpu34n"});
    });
});

let new_wallet = async function() {
    let w = await wallet.newWallet("did:xyz:ulapcuhsatnpuhza930hpu34n", "superbpass");
    let populated = await wallet.newKey(
        w,
        "ulapcuhsatnpuhza930hpu34n",
        "superbpass",
        "X25519KeyAgreementKey2019",
        "did:xyz:ulapcuhsatnpuhza930hpu34n");
    return populated;
};

let new_sealed_message = async function(message: string, header: string, ew: string, id: string, pass: string) {
    let s_header = JSON.stringify(header);
    // Act
    let sealed = await wallet.sealDidcommMessage(
        ew,
        "did:xyz:ulapcuhsatnpuhza930hpu34n",
        "superbpass",
        "ulapcuhsatnpuhza930hpu34n",
        message,
        s_header,
    );
    return sealed;
};

let new_header = function(message: string) {
    let m = JSON.parse(message);
    let header: DidcommHeader = {
        id: m.id,
        type: m.type,
        to: ["did::xyz:34r3cu403hnth03r49g03", "did:xyz:30489jnutnjqhiu0uh540u8hunoe"],
        from: "did:xyz:ulapcuhsatnpuhza930hpu34n"
    };
    return header;
}
