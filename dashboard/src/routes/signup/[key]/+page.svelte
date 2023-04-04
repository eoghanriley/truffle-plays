<script lang="ts">
	import type { ActionData } from './$types';
	export let form: ActionData;
	import '../../../app.css';
	import Modal from '$lib/components/Modal.svelte';

	let modals = [form?.incorrect, form?.key_incorrect];

	function toggleModal(event) {
		if (event.detail.name === 'credentials') {
			modals[0] = !modals[0];
		} else {
			modals[1] = !modals[1];
		}
	}
</script>

<main class="min-h-screen bg-slate-900">
	{#if modals[0]}
		<Modal
			name="credentials"
			title="Error Signing Up"
			body="There was an error please try again."
			button="Try again"
			on:toggleModal={toggleModal}
		/>
	{/if}

	{#if modals[1]}
		<Modal
			name="key"
			title="Nice Try :)"
			body="There was an error with the key you provided."
			button="Try again"
			on:toggleModal={toggleModal}
		/>
	{/if}

	<div class="mx-auto md:w-96 w-80 md:pt-8 pt-12">
		<div class="flex justify-center rounded-lg bg-slate-700 text-slate-50">
			<form method="POST" class="flex flex-col justify-start pt-6 pl-3 pb-6">
				<h2 class="text-3xl font-extrabold">Register</h2>
				<!--orgId-->
				<label for="orgId" class="pt-4 text-2xl font-bold">orgId</label>
				<input
					type="text"
					name="orgId"
					placeholder="orgId"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-600 rounded-md text-xl font-semibold"
				/>

				<!--Stream-->
				<label for="orgId" class="pt-3 text-2xl font-bold">Stream</label>
				<input
					type="text"
					name="stream"
					placeholder="Ludwig"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-600 rounded-md text-xl font-semibold"
				/>

				<!--Password-->
				<label for="password" class="pt-3 text-2xl font-bold">Password</label>
				<input
					type="password"
					name="password"
					placeholder="&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-600 rounded-md text-base font-semibold"
				/>

				<!--Confirm Password-->
				<label for="confirmPassword" class="pt-3 text-2xl font-bold">Confirm Password</label>
				<input
					type="password"
					name="confirmPassword"
					placeholder="&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-600 rounded-md text-base font-semibold"
				/>

				<!--Submit-->
				<input
					type="submit"
					value="Submit"
					class="mt-5 pt-1 w-28 rounded-md text-xl hover:border-2 hover:border-slate-50 bg-gradient-to-r from-red-400 via-emerald-400 to-sky-900 font-extrabold"
				/>
			</form>
		</div>
	</div>
</main>
