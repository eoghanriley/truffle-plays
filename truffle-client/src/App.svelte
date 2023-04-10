<script lang="ts">
  import { embed, user as userClient } from "@trufflehq/sdk";
  import { onMount } from "svelte";

  embed.setSize("400px", "300px");
  embed.setPosition("20px", "100px");
  if (!document.referrer) {
    embed.hide();
  }

  const getStreamers = async () => {
    const res = await fetch("http://localhost:3000/get_streams").then(
      async (response) => {
        return await response.json();
      }
    );
    activeStreamers = res.names;
  };

  let userId = "1";
  let activeStreamers;
  onMount(async () => {
    //let user = userClient.observable.subscribe({
    //  next: (user) => {
    //    userId = user.id;
    //  },
    //  error: (error) => {
    //    console.error(error);
    //  },
    //  complete: () => {},
    //});

    getStreamers();
  });

  let url = "http://localhost:3000/push";
  let streamer = "";
  let toggled = true;

  async function post(key: string) {
    if (streamer !== "") {
      let res = await fetch(url, {
        method: "POST",
        headers: {
          "Content-type": "application/json",
        },
        body: JSON.stringify({
          org_id: userId,
          input: key,
          stream: streamer,
        }),
      }).then((response) => {
        return response.json();
      });

      if (res.body !== "Success") {
        alert(res.error);
      }
    } else {
      alert("You need to select a streamer from the dropdown first!");
    }
  }

  function toggleController() {
    toggled = !toggled;
    if (toggled) {
      embed.setSize("400px", "300px");
    } else {
      embed.setSize("32px", "34px");
    }
  }

  toggleController();

  $: console.log(activeStreamers, typeof activeStreamers);
</script>

<main
  class="bg-sky-500 h-full w-full rounded-xl border border-slate-900 grid grid-cols-4"
>
  {#if toggled === true}
    <div class="dpad-1 m-6 absolute">
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-9 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d1_up")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-9 my-16 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d1_down")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-[4.5rem] my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d1_right")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d1_left")}
      />
    </div>

    <div class="button-set-1 mx-64 my-6 absolute">
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-9 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("x1")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-9 my-16 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("b1")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-[4.5rem] my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("a1")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("y1")}
      />
    </div>

    <div class="dpad-2 mx-64 my-44 absolute">
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-9 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d2_up")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-9 my-16 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d2_down")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md mx-[4.5rem] my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d2_right")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-md my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("d2_left")}
      />
    </div>

    <div class="button-set-2 my-44 absolute mx-6">
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-9 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("x2")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-9 my-16 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("b2")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full mx-[4.5rem] my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("a2")}
      />
      <button
        class="bg-stone-900 w-9 h-9 rounded-full my-8 absolute hover:border-2 hover:border-slate-300"
        on:click={() => post("y2")}
      />
    </div>

    <div>
      {#if typeof activeStreamers === undefined}
        <p>Loading streamers...</p>
      {:else}
        <select
          class="mx-[8rem] w-4/12 mt-[16.75rem] absolute bg-stone-800 hover:text-slate-50 text-slate-500 rounded-md p-1"
          bind:value={streamer}
        >
          {#each activeStreamers as streamer}
            <option value={streamer.stream}>{streamer.stream}</option>
          {/each}
        </select>
      {/if}
    </div>
  {/if}

  <button class="mx-[22.5] m-1 absolute " on:click={toggleController}>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      fill="currentColor"
      class="w-6 h-6"
    >
      <path
        fill-rule="evenodd"
        d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z"
        clip-rule="evenodd"
      />
    </svg>
  </button>
</main>
