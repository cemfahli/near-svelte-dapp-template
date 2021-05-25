<script>
	import { near, wallet, account, contract } from "./store";
	import Counter from "./components/Counter.svelte";
	import Header from "./components/Header.svelte";
	import Loading from './components/Loading.svelte';

	const unsubscribe = near.subscribe(() => console.log("Connected to NEAR"));

</script>

<Header></Header>
<main class="font-mono h-3/4 md:h-auto">
	<h1 class="text-5xl">TEMPLATE</h1>
	<h2 class="text-2xl">CONNECTION TO NEAR: 
		{#if $near}
			OK
		{:else}
			<Loading></Loading>
		{/if}
	</h2>
	<h2 class="text-2xl">WALLET: 
		{#if $wallet}
			OK
		{:else}
			<Loading></Loading>
		{/if}
	</h2>
	<h2 class="text-2xl">SIGNED IN: 
		{#if $wallet && $wallet.getAccountId()}
			{$wallet.getAccountId()}
		{:else}
			<Loading></Loading>
		{/if}
	</h2>
	<h2 class="text-2xl">ACCOUNT: 
		{#if $account}
			OK
		{:else}
			<Loading></Loading>
		{/if}
	</h2>
	<h2 class="text-2xl">CONTRACT: 
		{#if $contract}
			OK
		{:else}
			<Loading></Loading>
		{/if}
	</h2>
	{#if $contract}
		<Counter></Counter>
	{/if}
</main>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	body {
		padding: 0;
	}

	.btn-purple {
		@apply transition-colors rounded p-2 bg-purple-700 text-purple-100 hover:bg-purple-100 hover:border-purple-700 hover:text-purple-700;
	}
</style>