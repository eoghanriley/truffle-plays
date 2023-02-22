<script lang="ts">
  import { embed } from "@trufflehq/sdk";

  embed.setSize("450px", "350px");
  embed.setPosition("20px", "100px");
  if (!document.referrer) {
    embed.hide();
  }

  export let url = "http://127.0.0.1:3000/";

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

<main class="bg-sky-500 h-full w-full rounded-xl border border-slate-900">
  <div class="dpad-1">
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-6 mx-16 absolute"
      on:click={() => post("d1-up")}
    />
    <button class="bg-stone-900 w-9 h-9 rounded-md my-24 mx-16 absolute" />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[3.75rem] mx-[6.5rem] absolute"
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[3.75rem] mx-[1.45rem]  absolute"
    />
  </div>

  <div class="button-set-1">
    <button class="bg-stone-900 w-9 h-9 rounded-full my-6 mx-80 absolute" />
    <button class="bg-stone-900 w-9 h-9 rounded-full my-24 mx-80 absolute" />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-[3.75rem] mx-[22.5rem] absolute"
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-[3.75rem] mx-[17.45rem]  absolute"
    />
  </div>

  <div class="dpad-2">
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[13.5rem] mx-80 absolute"
    />
    <button class="bg-stone-900 w-9 h-9 rounded-md my-72 mx-80 absolute" />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[15.75rem] mx-[22.5rem] absolute"
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[15.75rem] mx-[17.45rem]  absolute"
    />
  </div>

  <div class="button-set-2">
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-[13.5rem] mx-16 absolute"
    />
    <button class="bg-stone-900 w-9 h-9 rounded-full my-72 mx-16 absolute" />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-[15.75rem] mx-[6.5rem] absolute"
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-full my-[15.75rem] mx-[1.45rem]  absolute"
    />
  </div>
</main>
