<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import CodeMirror from "./CodeMirror.svelte";

 // (async () => {
 //  let note = Object.assign(new backend.Note(), { text: "Bob" });

 //  let rowid = await backend.insert_note(note);
 //  console.log("Inserted rowid ", rowid);
 // })();
</script>

{#await backend.note_get(1n) then note}
 <div class="bg-black text-gray-500 flex w-screen min-h-screen max-h-screen">
  <div class="overflow-scroll max-h-screen w-full">
   <CodeMirror
    doc={note.text}
    on:docChanged={(e) => backend.note_update(1n, e.detail)}
   />
  </div>
 </div>
{:catch error}
 Error: {error}
{/await}
