<script lang="ts">
    import {createEventDispatcher} from "svelte";
    import Card from '../models/Card'

    const dispatch = createEventDispatcher();
    export let card: Card;
    const cardSelected = () => {
        dispatch('click', card)
    }
</script>

<article class="card-card" on:click={cardSelected}>
    <div class="card-card-content">
        <h3>{card.title}</h3>
        <ul>
            {#each card.options as option}
                <li>{option.text}</li>
            {/each}
        </ul>
    </div>
</article>

<style>
    .card-card {
        width: 24rem;
    }

    .card-card-content {
        padding: 1rem;
    }

    .card-card-content h3 {
        padding: 1rem 0;
    }
</style>
