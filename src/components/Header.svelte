<script>

	import { wallet } from "../store";
    const contractName = process.env.CONTRACT_ACCOUNT;
    const appName = process.env.APP_NAME;


	function signIn() {
		$wallet.requestSignIn(
			contractName,
			appName
		)
	}

	function signOut() {
		$wallet.signOut();
		window.location.replace(window.location.origin + window.location.pathname);
	}
</script>
<main>
    <div class="flex justify-between items-center border-b-2 p-3 bg-purple-700 md:px-5">
        <div class="text-purple-100 font-mono text-lg">Cem's Dapps!</div>

		{#if $wallet && !$wallet.isSignedIn()}
			<button class="btn-purple" on:click={signIn}>Sign In!</button>
		{:else}
			<button class="btn-purple" on:click={signOut}>Sign Out!</button>
		{/if}
    </div>
</main>