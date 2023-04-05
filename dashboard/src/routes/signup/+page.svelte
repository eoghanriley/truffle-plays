<script lang="ts">
	import '../../app.css';
	import { goto } from '$app/navigation';
	import Header from '$lib/components/Header.svelte';

	let key = '';

	function goToSignup() {
		console.log(key);
		goto(`/signup/${key}`);
	}
</script>

<main class="bg-slate-900 min-h-screen">
	<Header />
	<h2 class="text-6xl text-center font-extrabold text-slate-100 pt-8">How to Signup</h2>

	<p class="text-slate-100 text-w-prose text-center mt-8 text-xl md:mx-48 mx-10">
		<b class="font-extrabold text-2xl text-red-600"
			>NOTE: This is only for streamers if you are a viewer you do not need an account.</b
		>
		<br />In order to signup you need to get a key which you can get from me. In order to get the
		key you can dm me on twitter
		<i
			class="font-bold underline text-transparent bg-clip-text bg-gradient-to-r from-red-400 from-40% via-emerald-400 via-60% to-sky-600 to-90%"
			><a href="https://twitter.com/Seniorquesopan1" target="_blank">@Seniorquesopan1</a></i
		>
		or by emailing
		<i
			class="font-bold underline text-transparent bg-clip-text bg-gradient-to-r from-red-400 from-40% via-emerald-400 via-60% to-sky-600 to-90%"
			><a href="mailto:truffle-plays@eoghanriley.dev">truffle-plays@eoghanriley.dev</a></i
		>. If you email me I will need you to provide proof that you are who you say you are as a
		security measure. Once you have the key enter it in below.
	</p>

	<div class="flex justify-center">
		<div class="flex flex-col">
			<label for="key" class="mt-8 text-3xl font-bold text-slate-300 text-center">Key:</label>
			<form class="flex" on:submit={goToSignup}>
				<input
					type="text"
					name="key"
					placeholder="PINK-FLUFFY-UNICORNS"
					autocomplete="off"
					required
					bind:value={key}
					class="max-w-xs bg-slate-600 text-slate-50 rounded-md text-xl font-semibold text-center mt-2"
				/>

				<button type="submit" class="w-7 h-7 bg-green-400 rounded-lg ml-3 mt-2"
					><a href="/signup/{key}">-></a></button
				>
			</form>
		</div>
	</div>
</main>
