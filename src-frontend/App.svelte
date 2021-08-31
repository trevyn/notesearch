<script>
 import * as backend from "./turbocharger_generated";
 import CodeMirror from "./CodeMirror.svelte";

 (async () => {
  let person = Object.assign(new backend.Person(), { name: "Bob" });

  // let person2 = new backend.Person({ name: "bob" });

  let rowid = await backend.insert_person(person);
  console.log("Inserted rowid ", rowid);
 })();
</script>

Hello!<br />

{#await backend.get_person(1n) then person}
 Name: {person.name}
{:catch error}
 Error: {error}
{/await}

<div class="bg-black text-gray-500 flex w-screen min-h-screen max-h-screen">
 <div class="overflow-scroll max-h-screen w-2/5">
  <CodeMirror doc="hello" />
 </div>
</div>
