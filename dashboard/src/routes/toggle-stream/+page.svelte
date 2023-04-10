<script lang="ts">
	import type { PageData, ActionData } from './$types';
	import Modal from '$lib/components/Modal.svelte';
	import Header from '$lib/components/Header.svelte';
	import '../../app.css';

	export let data: PageData;
	export let form: ActionData;

	let modals = [form?.incorrect];
	function toggleModal(event) {
		modals[0] = !modals[0];
	}
</script>

<main class="min-h-screen bg-slate-900">
	<Header />

	{#if modals[0]}
		<Modal
			name="error"
			title="Error Toggling Stream"
			body="There was an error please try again."
			button="Try again"
			on:toggleModal={toggleModal}
		/>
	{/if}

	<div class="text-slate-100 text-center text-2xl pt-8">
		{#if data.active}
			<p>Your stream is currently <b class="font-extrabold uppercase">active</b>.</p>
		{:else}
			<p>Your stream is currently <b class="font-extrabold uppercase">inactive</b>.</p>
		{/if}

		<p class="text-xl mt-2">In order to change this enter your api-token below.</p>
		<p class="text-base mt-1">
			If you want to recieve inputs then make your stream active, and if you don't then make it
			inactive.
		</p>
	</div>

	<div class="flex justify-center">
		<div class="flex flex-col">
			<label for="key" class="mt-8 text-3xl font-bold text-slate-300 text-center">Key:</label>
			<form method="POST" class="flex">
				<input
					type="password"
					name="token"
					placeholder="&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-600 text-slate-50 rounded-md text-base font-semibold text-center mt-2"
				/>

				<button type="submit" class="w-7 h-7 bg-green-400 rounded-lg ml-3 mt-2">-></button>
			</form>
		</div>
	</div>
</main>
