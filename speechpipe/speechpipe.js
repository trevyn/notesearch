const speech = require("@google-cloud/speech");
const client = new speech.SpeechClient({
 projectId: "equi-325104",
 keyFilename: "/Users/e/Downloads/google-cloud.json",
});

const recognizeStream = client
 .streamingRecognize({
  config: {
   encoding: "LINEAR16",
   sampleRateHertz: 48000,
   languageCode: "en-US",
   maxAlternatives: 30,
   enableWordConfidence: true,
   enableWordTimeOffsets: true,
   // model: "video",
  },
  interimResults: true,
 })
 .on("error", console.error)
 // .on("end", () => console.log("ended"))
 .on("data", (data) => process.stdout.write(JSON.stringify(data) + "\n\n"));

// console.log("start");

process.stdin.pipe(recognizeStream);
