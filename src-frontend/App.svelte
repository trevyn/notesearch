<script lang="ts">
 import * as backend from "./turbocharger_generated";
 import CodeMirror from "./CodeMirror.svelte";

 (async () => {
  let note = Object.assign(new backend.Note(), { text: "Bob" });

  let rowid = await backend.insert_note(note);
  console.log("Inserted rowid ", rowid);
 })();
</script>

Hello!<br />

{#await backend.get_note(1n) then note}
 Name: {note.text}
{:catch error}
 Error: {error}
{/await}

<div class="bg-black text-gray-500 flex w-screen min-h-screen max-h-screen">
 <div class="overflow-scroll max-h-screen w-2/5">
  <CodeMirror doc="hello" />
 </div>
</div>
