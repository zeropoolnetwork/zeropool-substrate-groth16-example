const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api'); //requirement for nodejs

const WS_URL = 'ws://127.0.0.1:9944';
const ADDR = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const PHRASE = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
const VK = 'yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh';
const INVALID_VK = '1234';
// groth16 verification passing proof and input (verification key is stored)
const PROOF_INPUT ='{"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
const INVALID_PROOF_INPUT = '{"proof":"AQexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';

// account address of the well-know testing account "Alice" generated automatically with some funds
function initAccount() {
  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri(PHRASE, { name: 'Alice' });

  return alice;
}

describe('ZeroPool', () => {
  let wsProvider;
  let api;

  beforeAll(async () => {
    wsProvider = new WsProvider(WS_URL);
    api = await ApiPromise.create({ provider: wsProvider });
  });

  test('get genesis hash', () => {
    expect(api.genesisHash).toBeTruthy();
  });

  test('get balance', async () => {
    const { nonce, data: balance } = await api.query.system.account(ADDR);
    expect(balance).toBeTruthy();
  });

  test('get last block mined', async () => {
    const lastHeader = await api.rpc.chain.getHeader();
    expect(lastHeader.hash).toBeTruthy();
  });

  describe('setVk', () => {
    it('succeeds with valid VK', async () => {
      const alice = initAccount();
      await new Promise(async resolve => {
        const unsub = await api.tx.zeropool.setVk(VK).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'VerificationKeyUpdated');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });

    it('fails with invalid VK', async () => {
      const alice = initAccount();
      await new Promise(async resolve => {
        const unsub = await api.tx.zeropool.setVk(INVALID_VK).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'ExtrinsicFailed');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });
  });

  describe('testGroth16Verify', () => {
    // set the verification key for testing
    beforeAll(async () => {
      const alice = initAccount();
      await new Promise(async resolve => {
        const unsub = await api.tx.zeropool.setVk(VK).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'VerificationKeyUpdated');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });

    it('succeeds with valid proof', async () => {
      const alice = initAccount();

      await new Promise(async resolve => {
        // groth16 verification passing proof and input (verification key is stored)
        const unsub = await api.tx.zeropool.testGroth16Verify(PROOF_INPUT).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'VerificationSuccessful');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });

    it('fails with invalid proof', async () => {
      const alice = initAccount();

      await new Promise(async resolve => {
        const unsub = await api.tx.zeropool.testGroth16Verify(INVALID_PROOF_INPUT).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'VerificationFailed');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });

    it('fails with bad input', async () => {
      const alice = initAccount();

      await new Promise(async resolve => {
        const randomData = Array(99).fill().map(() => Math.round(Math.random() * 255));

        const unsub = await api.tx.zeropool.testGroth16Verify(randomData).signAndSend(alice, ({ events }) => {
          const event = events.find(({ event: { method }}) => method == 'VerificationFailed');

          if (event) {
            unsub();
            resolve();
          }
        });
      });
    });
  });
});
