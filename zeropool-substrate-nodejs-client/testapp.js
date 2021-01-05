// Testing Unit to interact with zeropool-substrate pallet
// Import Polkadot Library (to install: yarn add @polkadot/api)

const { ApiPromise, WsProvider } = require('@polkadot/api');    //requirement for nodejs
const { Keyring } = require('@polkadot/api');
// Construct web socket interface, to use secure socket replace ws:// with wss://
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
//call the testing unit that must be a function defined as "async" to use "await"
let testPassed=0;
let testFailed=0;
unit_testing();


async function unit_testing(){
    const api = await ApiPromise.create({ provider: wsProvider });      // create API object
    let genesisHash=api.genesisHash.toHex();
    if(genesisHash!=undefined){
        console.log("Test #1 - Get genesis hash:"+genesisHash+" (Passed)"); //read the genesis hash
        testPassed++;
    }else{
        console.log("Test #1 - Get genesis hash:"+genesisHash+" (Failed)"); 
        testFailed++;
    }
    const ADDR='5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';      // account address of the well-know testing account "Alice" generated automatically with some funds
    console.log("Test #2/1 - Get balance of the account (in Progress)");              
    const { nonce, data: balance } = await api.query.system.account(ADDR);  //read the balance of the account
    if(balance.free!=undefined){
          console.log(`Test #2/2 - Balance of ${balance.free} and a nonce of ${nonce} (Passed)`);
          testPassed++;
    }else{
          console.log(`Test #2/2 - Balance not available (Failed)`);
          testFailed++;
    }
    console.log("Test #3/1 - Get last block mined and its hash (in Progress");
    const lastHeader = await api.rpc.chain.getHeader();                 // get last block number
    if(lastHeader!=undefined){
        console.log(`Test #3/2  last block #${lastHeader.number} has hash ${lastHeader.hash} (Passed)`);
        testPassed++;
    }else{
        console.log(`Test #3/2  last block not available (Failed)`);
        testFailed++;
    }
    console.log("Test #4/1 - Adding test account Alice (in Progress)");               // generate key pairs starting from the seed
    const PHRASE = 'bottom drive obey lake curtain smoke basket hold race lonely fit walk//Alice';
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri(PHRASE,{ name: 'Alice' });
    if(alice!=undefined){
      console.log(`Test #4/2 - ${alice.meta.name}: has address ${alice.address} with publicKey [${alice.publicKey}] (in Progress)`);
      console.log("Test #4/3 - Keyring for account Alice has been set (Passed");
      testPassed++;
    }else {
      console.log(`Test #4/2 - Account creation did not work (Failed)`);
      testFailed++;
    }
    console.log("Test #5/1 - Set Verification Key (in Progress)");                  // set the verification key for testing
    let vk="yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh";
    const unsubv = await api.tx.zeropool.setVk(vk).signAndSend(alice,async ({ status, events, dispatchError })  => {
        if (dispatchError) {      // check for immediate verification success
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { documentation, name, section } = decoded;
            console.log(`Test #5/2  Set Verification Key did not work - ${section}.${name}: ${documentation.join(' ')} (Failed)`);
            testFailed++;
          } else {
            // Other, CannotLookup, BadOrigin, no extra info
            console.log(`Test #5/2 Set Verification Key - FAILED ${dispatchError.toString()} (Passed)`);
            testPassed++;
          }
          unsubv();
        }
        console.log(`Test #5/3 Setting Verification Key - Status - ${status} (in Progress)`);
        if (status.isInBlock) {
          console.log(`Test #5/4 - Setting Verification Key - Transaction included at blockHash ${status.asInBlock} (Passed)`);
          testPassed++;
        } else if (status.isFinalized) {
          console.log(`Test #5/5 - Setting Verification Key - Transaction finalized at blockHash ${status.asFinalized} (Passed)`);
          testPassed++;
          events.forEach(({ phase, event: { data, method, section } }) => {
            console.log(`Test #5/6 - Setting Verification Key- SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data} (Passed)`);
            testPassed++;
          });
          unsubv();
          let = proofinput='{"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
          let result=await testGroth16Verify(alice,proofinput); // make groth16 verification
          if(result){
              console.log("Test #12/1 - testGroth16Verify SUCCEDED (Passed");
              testPassed++;
          }
          else{
              console.log("Test #12/1 - testGroth16Verify FAILED");
              testFailed++;
          }
        }
  

    });
}
// function to make the groth16 verification on static example
async function testGroth16Verify(keyring,proofinput) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  console.log("Test #6/1 - Groth16 Verify (in Progress)");  
  let dispatchErrorG=""; // Global vars
  let dispatchInfoG="";         
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(keyring,({ dispatchError,dispatchInfo,status, events }) => {
      if (dispatchError) {      // check for immediate verification success
        dispatchErrorG=dispatchError;
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          console.log(`Test #6/2 Groth16 Verification thrown an errror - ${section}.${name}: ${documentation.join(' ')} (Failed)`);
          testFailed++;
        } else {
          console.log(`Test #6/2 Groth16 Verification thrown an error ${dispatchError.toString()} (Failed)`);
          testFailed++;
        }
        unsubg();
        process.exit(1);
      }else{
        console.log(`Test #6/2 Groth16 Verification - Status - ${status}(in Progress)`);
        if(dispatchInfo!=undefined)
          dispatchInfoG=dispatchInfo;
      }
      if (status.isInBlock) {
        console.log(`Test #6/3 - Groth16 Verification - SUCCEDED - Transaction included at blockHash ${status.asInBlock} (Passed)`);
        testPassed++;
      } else if (status.isFinalized) {
        console.log(`Test #6/4 - Groth16 Verification - SUCCEDED - Transaction finalized at blockHash ${status.asFinalized} (Passed)`);
        testPassed++;
        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log(`Test #6/5 - Groth16 Verification - SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data} (Passed)`);
          testPassed++;
        });
        unsubg();
        testFailedGroth16Verify(keyring,proofinput);
      }
  });
  // waiting for result
  const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay));
  const repeatedWait = async () => {
    console.log("Test #6/6 - Waiting for result or TIMEOUT (in Progress)");
    await sleep(2000);
    if(dispatchErrorG.length==0 && dispatchInfoG.length==0){
      console.log("Test #6/7 - Waiting for result or TIMEOUT (in Progress)");
      await sleep(2000);
    }
    if(dispatchErrorG.length==0 && dispatchInfoG.length==0){
      console.log("Test #6/8 - Waiting for result or TIMEOUT (in Progress)");
      await sleep(2000);
    }
    if(dispatchErrorG.length==0 && dispatchInfoG.length==0){
      console.log("Test #6/9 - Waiting for result or TIMEOUT (in Progress)");
      await sleep(2000);
      console.log("Test #6/10 - TIMEOUT REACHED (Failed");
      testFailed++;
    }
  };
  await repeatedWait();
  //returning the result
  let rx=true;
  if (dispatchErrorG.length==0 && dispatchInfoG.length>0){
      console.log(`Test #6/11 - Blockchain has been written - DispatchInfo: ${dispatchInfoG} (Passed)`);
      rx=true;
      testPassed++;
  }
  if (dispatchErrorG.length>0 && dispatchInfoG.length>0){
      console.log(`Test #6/11 - Blockchain has NOT been written: ${dispatchErrorG} (Failed)`);
      rx=false;
      testFailed++;
  }
  return(rx);
}

// function to make the groth16 verification on static example (failing tests)
async function testFailedGroth16Verify(keyring,proofinputcallback) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  let proofinput='{"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  //to simulate an error use a wrong proof remove // in the following line
  proofinput='{"proof":"AQexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  console.log("Test #7/1 - Groth16 Verify - SIMULATED ERROR (in Progress)");  
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(keyring,({ dispatchError,dispatchInfo,status, events }) => {
      if (dispatchError) {      // check for immediate verification success
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          console.log(`Test #7/2 Groth16 Verification, generated failure - ${section}.${name}  ${documentation.join(' ')} - DispatchInfo: ${dispatchInfo} (Passed)`);
          testPassed++;
        } else {
          console.log(`Test #7/2 Groth16 Verification, generated failure - reason: ${dispatchError.toString()} - DispatchInfo: ${dispatchInfo} (Passed)`);
          testPassed++;
        }
        unsubg();
        // execute the verification  getting result by a call back
        testGroth16VerifyCallBack(keyring,proofinputcallback,receiveresult);
      }else{
        // this part will not be really executed because it's a simulated error
        console.log(`Test #7/2 Groth16 Verification - Status - ${status} (in Progress)`);
      }
      if (status.isInBlock) {
        console.log(`Test #7/3 - Groth16 Verification - SUCCEDED - Transaction included at blockHash ${status.asInBlock} (Passed)`);
        testPassed++;
      } else if (status.isFinalized) {
        console.log(`Test #7/4 - Groth16 Verification - SUCCEDED - Transaction finalized at blockHash ${status.asFinalized} (Failed)`);
        testPassed++;
        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log(`Test #7/5 - Groth16 Verification - SUCCEDED - Events: ${phase}: ${section}.${method}:: ${data} (Failed)`);
          testFailed++;
        });
        unsubg();
        process.exit(1);
      }
  });
}
// function to make the groth16 verification with a callback to return the result
async function testGroth16VerifyCallBack(keyring,proofinput,callbackfunction) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(keyring,({ dispatchError,dispatchInfo,status, events }) => {
      if (dispatchError) {      // check for immediate verification success
        let msgerror="";
        dispatchErrorG=dispatchError;
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          msgerror=`KO - Groth16 Verification - FAILED - ${section}.${name}: ${documentation.join(' ')}`;
          testFailed++;
        } else {
          msgerror=`KO - Groth16 Verification - FAILED - ${dispatchError.toString()}`;
          testFailed++;
        }
        unsubg();
        callbackfunction(false,msgerror,keyring,proofinput);
      }
      if (status.isInBlock) {
        unsubg();
        callbackfunction(true,"OK - Groth16 Verification Successful",keyring, proofinput);
      }
  });
}
// callback function to receive result from testGroth16VerifyCallBack
function receiveresult(result,msg,keyring,proofinputcallback){
  console.log("Test #10/1 - Groth16 Verification with Callback - result:" +result+" msg:"+msg+" (Passed)");
  testPassed++;
  //we simulate an error with call back
  testFailedGroth16VerifyCallBack(keyring,proofinputcallback,failedreceiveresult);
}
// function to make the groth16 verification with a callback to return the result (simulated error)
async function testFailedGroth16VerifyCallBack(keyring,proofinput,callbackfunction) {
  const api = await ApiPromise.create({ provider: wsProvider });      // create API object
  // groth16 verification passing proof and input (verification key is stored)
  // we set a wrong proofinput to simulate the error
  proofinput='{"proof":"AQexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}';
  const unsubg = await api.tx.zeropool.testGroth16Verify(proofinput).signAndSend(keyring,({ dispatchError,dispatchInfo,status, events }) => {
      if (dispatchError) {      // check for immediate verification success
        let msgerror="";
        dispatchErrorG=dispatchError;
        if (dispatchError.isModule) {
          // for module errors, we have the section indexed, lookup
          const decoded = api.registry.findMetaError(dispatchError.asModule);
          const { documentation, name, section } = decoded;
          msgerror=`KO - Groth16 Verification - FAILED - ${section}.${name}: ${documentation.join(' ')}`;
          testPassed++;
        } else {
          msgerror=`KO - Groth16 Verification - FAILED - ${dispatchError.toString()}`;
          testPassed++;
        }
        unsubg();
        callbackfunction(false,msgerror);
      }
      if (status.isInBlock) {
        unsubg();
        callbackfunction(true,"OK - Groth16 Verification Successful (Failed)");
        testFailed++;
      }
  });
}
// callback function to receive result from testGroth16VerifyCallBack
function failedreceiveresult(result,msg){
  console.log("Test #11/1 - Groth16 Verification simulated error with Callback - result:" +result+" msg:"+msg+" (Passed)");
  console.log("*******************************************");
  console.log("Total tests PASSED ("+testPassed+")");
  console.log("Total tests FAILED ("+testFailed+")");
  console.log("*******************************************");
  process.exit(1);
}



