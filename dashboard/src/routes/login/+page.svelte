<script lang="ts">
	import '../../app.css';
	import type { ActionData } from './$types';
	import Header from '$lib/components/Header.svelte';

	export let form: ActionData;

	let modal = form?.incorrect;

	function toggleModal() {
		modal = !modal;
	}
</script>

<main class="min-h-screen bg-slate-900">
	<Header />
	{#if modal}
		<div
			class="relative z-10"
			aria-labelledby="Error with credentials"
			role="dialog"
			aria-modal="true"
		>
			<div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" />

			<div class="fixed inset-0 z-10 overflow-y-auto">
				<div
					class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0"
				>
					<div
						class="relative transform overflow-hidden rounded-lg bg-slate-700 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-lg"
					>
						<div class="bg-slate-800 px-4 pb-4 pt-5 sm:p-6 sm:pb-4">
							<div class="sm:flex sm:items-start">
								<div
									class="mx-auto flex h-12 w-12 flex-shrink-0 items-center justify-center rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10"
								>
									<svg
										class="h-6 w-6 text-red-600"
										fill="none"
										viewBox="0 0 24 24"
										stroke-width="1.5"
										stroke="currentColor"
										aria-hidden="true"
									>
										<path
											stroke-linecap="round"
											stroke-linejoin="round"
											d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
										/>
									</svg>
								</div>
								<div class="mt-3 text-center sm:ml-4 sm:mt-0 sm:text-left">
									<h3 class="text-base font-semibold leading-6 text-slate-100" id="modal-title">
										Error with credentials
									</h3>
									<div class="mt-2">
										<p class="text-sm text-slate-100">
											There was an issue with the credentials entered please try again.
										</p>
									</div>
								</div>
							</div>
						</div>
						<div class="bg-slate-800 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6">
							<button
								type="button"
								class="mt-3 inline-flex w-full justify-center rounded-md bg-slate-700 px-3 py-2 text-sm font-semibold text-slate-200 shadow-sm ring-1 ring-inset ring-slate-300 hover:bg-slate-50 hover:text-slate-700 sm:mt-0 sm:w-auto"
								on:click={toggleModal}>Try again</button
							>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<div class="mx-auto md:w-96 w-80 md:pt-8 pt-12">
		<div class="flex justify-center rounded-lg bg-slate-800 text-slate-50">
			<form method="POST" class="flex flex-col justify-start pt-6 pl-3 pb-6">
				<h2 class="text-3xl font-extrabold">Login</h2>
				<!--orgId-->
				<label for="orgId" class="pt-4 text-2xl font-bold">orgId</label>
				<input
					type="text"
					name="orgId"
					placeholder="orgId"
					required
					class="max-w-xs bg-slate-700 rounded-md text-lg font-semibold"
				/>

				<!--Password-->
				<label for="password" class="pt-3 text-2xl font-bold">Password</label>
				<input
					type="password"
					name="password"
					placeholder="&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;&#9679;"
					autocomplete="off"
					required
					class="max-w-xs bg-slate-700 rounded-md text-base font-semibold"
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
