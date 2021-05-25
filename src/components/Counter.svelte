<script>
    //export let near;
    import Loading from './Loading.svelte';
    import { writable } from 'svelte/store';
    import { contract, wallet } from "../store";


    let count_promise;
    let current_count = writable(0);
    getCount();

    function getCount() {
        count_promise = $contract.get_num();
    }

    function decrement() {
        count_promise = $contract.decrement({}).then(() => getCount());
    }

    function increment() {
        count_promise = $contract.increment({}).then(() => getCount());
    }
</script>

<main>
    <div class="flex flex-col items-center p-4 md:px-4">
        <p>The count is: 
        {#await count_promise}
            <Loading></Loading> 
        {:then count}
            <span class="text-lg">{count}</span>
            {current_count.set(count)||''}
        {:catch error}
            <p style="color: red">{error.message}</p>
        {/await}
        </p>
        <div class="flex flex-row justify-evenly w-full">
            {#if $wallet && $wallet.isSignedIn()}
                <button class="btn-purple w-1/4" on:click={decrement} hidden={$current_count==0}>-</button>
            {/if}
            <button class="btn-purple" on:click={getCount}>Refresh</button>
            {#if $wallet && $wallet.isSignedIn()}
                <button class="btn-purple w-1/4" on:click={increment}>+</button>
            {/if}
        </div>
    </div>
</main>