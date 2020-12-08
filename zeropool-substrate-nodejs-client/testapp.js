// Testing Unit to interact with zeropool-substrate pallet
// Import Polkadot Library (to install: yarn add @polkadot/api)

const { ApiPromise, WsProvider } = require('@polkadot/api');    //requirement for nodejs
const { Keyring } = require('@polkadot/api');

// Construct web socket interface, to use secure socket replace ws:// with wss://
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
//call the testing unit that be a function defined as "async" to use "await"
unit_testing();


async function unit_testing(){
    const api = await ApiPromise.create({ provider: wsProvider });      // create API object
    console.log("Test #1 - Get genesis hash:"+api.genesisHash.toHex()); //read the genesis hash
    const ADDR='5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';      // account address of the well-know testing account "Alice" generated automatically with some funds
    console.log("Test #2/1 - Get balance of the account");              
    const { nonce, data: balance } = await api.query.system.account(ADDR);  //read the balance of the account
    console.log(`Test #2/2 - Balance of ${balance.free} and a nonce of ${nonce}`);
    console.log("Test #3/1 - Get last block mined");
    const lastHeader = await api.rpc.chain.getHeader();                 // get last block number
    console.log(`Test #3/2  last block #${lastHeader.number} has hash ${lastHeader.hash}`);
    console.log("Test #4/1 - Adding test account Alice");               // generate key pairs starting from the seed
    const PHRASE = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri(PHRASE,{ name: 'Alice' });
    console.log(`Test #4/2 - ${alice.meta.name}: has address ${alice.address} with publicKey [${alice.publicKey}]`);
    console.log("Test #4/3 - Keyring for account Alice has been set");
    console.log("Test #5/1 - Set Verification Key");                  // set the verification key for testing
    let vk="yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh";
    const unsubv = await api.tx.zeropool.setVk(vk).signAndSend(alice,({ status, events, dispatchError }) => {
        if (dispatchError) {      // check for immediate verification success
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { documentation, name, section } = decoded;
            console.log(`Test #5/2  Set Verification Key - FAILED - ${section}.${name}: ${documentation.join(' ')}`);
          } else {
            // Other, CannotLookup, BadOrigin, no extra info
            console.log(`Test #5/2 Set Verification Key - FAILED ${dispatchError.toString()}`);
        }
        unsubv();
      }
      console.log(`Test #5/2 Setting Verification Key - Status - ${status}`);
      if (status.isInBlock) {
          console.log(`Test #5/3 - Setting Verification Key - Transaction included at blockHash ${status.asInBlock}`);
      } else if (status.isFinalized) {
          console.log(`Test #5/4 - Setting Verification Key - Transaction finalized at blockHash ${status.asFinalized}`);
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`Test #5/5 - Setting Verification Key- SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data}`);
          });
          unsubv();
          testGroth16Verify(alice); // make groth16 verification
      }
    });
}
// function to make the groth16 verification on static example
async function testGroth16Verify(alice) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  let proofinput='{"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  //to simulate an error use a wrong proof remove // in the following line
  //proofinput='{"proof":"AQexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  console.log("Test #6/1 - Groth16 Verify");                
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(alice,({ status, events, dispatchError }) => {
      if (dispatchError) {      // check for immediate verification success
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          console.log(`Test #6/2 Groth16 Verification - FAILED - ${section}.${name}: ${documentation.join(' ')}`);
        } else {
          // Other, CannotLookup, BadOrigin, no extra info
          console.log(`Test #6/2 Groth16 Verification - FAILED ${dispatchError.toString()}`);
        }
        unsubg();
        process.exit(1);
      }else{
        console.log(`Test #6/2 Groth16 Verification - Status - ${status}`);
      }
      if (status.isInBlock) {
        console.log(`Test #6/3 - Groth16 Verification - SUCCEDED - Transaction included at blockHash ${status.asInBlock}`);
      } else if (status.isFinalized) {
        console.log(`Test #6/4 - Groth16 Verification - SUCCEDED - Transaction finalized at blockHash ${status.asFinalized}`);
        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log(`Test #6/5 - Groth16 Verification - SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data}`);
        });
        unsubg();
        testFailedGroth16Verify(alice);
      }
  });
}
// function to make the groth16 verification on static example
async function testFailedGroth16Verify(alice) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  let proofinput='{"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  //to simulate an error use a wrong proof remove // in the following line
  proofinput='{"proof":"AQexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  console.log("Test #7/1 - Groth16 Verify - SIMULATED ERROR");                
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(alice,({ status, events, dispatchError }) => {
      if (dispatchError) {      // check for immediate verification success
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          console.log(`Test #7/2 Groth16 Verification - FAILED - ${section}.${name}: ${documentation.join(' ')}`);
        } else {
          // Other, CannotLookup, BadOrigin, no extra info
          console.log(`Test #7/2 Groth16 Verification - FAILED ${dispatchError.toString()}`);
        }
        unsubg();
        process.exit(1);
      }else{
        console.log(`Test #7/2 Groth16 Verification - Status - ${status}`);
      }
      if (status.isInBlock) {
        console.log(`Test #7/3 - Groth16 Verification - SUCCEDED - Transaction included at blockHash ${status.asInBlock}`);
      } else if (status.isFinalized) {
        console.log(`Test #7/4 - Groth16 Verification - SUCCEDED - Transaction finalized at blockHash ${status.asFinalized}`);
        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log(`Test #7/5 - Groth16 Verification - SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data}`);
        });
        unsubg();
        process.exit(1);
      }
  });
}


