<script lang="ts">
  import { embed } from "@trufflehq/sdk";

  embed.setSize("400px", "300px");
  embed.setPosition("20px", "100px");
  if (!document.referrer) {
    embed.hide();
  }

  let url = "http://127.0.0.1:3000/";

  function post(key: string) {
    console.log(JSON.stringify({ input: key }));
    fetch(url, {
      method: "POST",
      headers: {
        accept: "application.json",
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ input: key }),
      cache: "default",
    });
  }
</script>

<main
  class="bg-sky-500 h-full w-full rounded-xl border border-slate-900 grid grid-cols-4"
>
  <div class="dpad-1 m-6 absolute">
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-9 absolute"
      on:click={() => post("d1_up")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-9 my-16 absolute"
      on:click={() => post("d1_down")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-[4.5rem] my-8 absolute"
      on:click={() => post("d1_right")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-8 absolute"
      on:click={() => post("d1_left")}
    />
  </div>

  <div class="button-set-1 mx-64 my-6 absolute">
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-9 absolute"
      on:click={() => post("x1")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-9 my-16 absolute"
      on:click={() => post("b1")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-[4.5rem] my-8 absolute"
      on:click={() => post("a1")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-8 absolute"
      on:click={() => post("y1")}
    />
  </div>

  <div class="dpad-2 mx-64 my-44 absolute">
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-9 absolute"
      on:click={() => post("d2_up")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-9 my-16 absolute"
      on:click={() => post("d2_down")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md mx-[4.5rem] my-8 absolute"
      on:click={() => post("d2_right")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-8 absolute"
      on:click={() => post("d2_left")}
    />
  </div>

  <div class="button-set-2 my-44 absolute mx-6">
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-9 absolute"
      on:click={() => post("x2")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-9 my-16 absolute"
      on:click={() => post("b2")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full mx-[4.5rem] my-8 absolute"
      on:click={() => post("a2")}
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-8 absolute"
      on:click={() => post("y2")}
    />
  </div>
</main>
