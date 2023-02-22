<script lang="ts">
  import { embed, org as orgClient } from "@trufflehq/sdk";

  embed.setSize("400px", "300px");
  embed.setPosition("20px", "100px");
  if (!document.referrer) {
    embed.hide();
  }

  let id: string;

  const subscription = orgClient.observable.subscribe({
    next: (org) => {
      id = org.id;
    },
    error: (error) => {
      console.error(error);
    },
    complete: () => {},
  });

  let url = "";
  let toggled = true;

  function post(key: string) {
    console.log(JSON.stringify({ input: key, id: id }));
    if (url !== "") {
      fetch(url, {
        method: "POST",
        headers: {
          accept: "application.json",
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ input: key }),
        cache: "default",
      });
    } else {
      alert("You need to set the url first!");
    }
  }

  function setUrl() {
    url = prompt("Set the url");
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
</script>

<main
  class="bg-sky-500 h-full w-full rounded-xl border border-slate-900 grid grid-cols-4"
>
  {#if toggled === true}
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

    <button class="mx-[23rem] mt-2 absolute" on:click={setUrl}>
      <svg
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
        class="w-6 h-6"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d="M13.19 8.688a4.5 4.5 0 011.242 7.244l-4.5 4.5a4.5 4.5 0 01-6.364-6.364l1.757-1.757m13.35-.622l1.757-1.757a4.5 4.5 0 00-6.364-6.364l-4.5 4.5a4.5 4.5 0 001.242 7.244"
        />
      </svg>
    </button>
  {/if}

  <button class="mx-[22.5] m-1 absolute" on:click={toggleController}>
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
