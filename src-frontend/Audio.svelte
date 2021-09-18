<script lang="ts">
 import { onMount, createEventDispatcher } from "svelte";

 const dispatch = createEventDispatcher();

 let node;
 let ctx;

 onMount(async () => {
  var ctx = node.getContext("2d");
  let audioCtx = new AudioContext();

  let stream = await navigator.mediaDevices.getUserMedia({
   video: false,
   audio: true,
  });

  let source = audioCtx.createMediaStreamSource(stream);
  let scriptNode = audioCtx.createScriptProcessor(2048, 1, 1);

  scriptNode.onaudioprocess = (audioProcessingEvent) => {
   let inputBuffer = audioProcessingEvent.inputBuffer;
   console.log(inputBuffer.sampleRate);
   let inputData = inputBuffer.getChannelData(0);

   ctx.clearRect(0, 0, 2048, 200);
   ctx.beginPath();
   ctx.moveTo(0, 100 + inputData[0] * 400);
   for (let x = 0; x < 2048; x++) {
    ctx.lineTo(x, 100 + inputData[x] * 400);
   }

   ctx.stroke();
   ctx.closePath();
  };

  source.connect(scriptNode);
  scriptNode.connect(audioCtx.destination);
 });
</script>

<canvas
 id="myCanvas"
 width="2048"
 height="200"
 style="border: 1px solid #000000; width: 1024px"
 bind:this={node}
/>
