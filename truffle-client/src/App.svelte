<script lang="ts">
  import { embed } from "@trufflehq/sdk";

  embed.setSize("450px", "350px");
  embed.setPosition("20px", "100px");
  if (!document.referrer) {
    embed.hide();
  }
</script>

<main class="bg-sky-500 h-full w-full rounded-xl">
  <div class="dpad-1">
    <button class="bg-stone-900 w-9 h-9 rounded-md my-6 mx-16 absolute" />
    <button class="bg-stone-900 w-9 h-9 rounded-md my-24 mx-16 absolute" />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[3.75rem] mx-[6.5rem] absolute"
    />
    <button
      class="bg-stone-900 w-9 h-9 rounded-md my-[3.75rem] mx-[1.45rem]  absolute"
    />
  </div>
</main>
