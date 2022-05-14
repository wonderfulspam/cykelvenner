<script lang="ts">
    import Card from './components/Card.svelte'
    import {onMount} from "svelte";

    let cards: Card[] = []

    onMount(async () => {
        const resp = await fetch('/api/cards')
        cards = await resp.json<Card>()
	console.log(cards)
    })

    let selectedCard: Card | null = null

    const onCardSelect = (selected: CustomEvent<Card>) => {
        console.log({selected})
        selectedCard = selected.detail
    }
</script>

<main>
    <h1>Here's some data</h1>

    <section class="cards">
        {#each cards as card}
            <Card on:click={onCardSelect} card={card} />
        {/each}
    </section>
</main>

<style>
    main {
        padding: 2rem;
    }

    main h1 {
        padding-bottom: 2rem;
    }

    .cards {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(24rem, 1fr));
        grid-column-gap: 1rem;
        grid-row-gap: 2rem;
        justify-items: center;
    }
</style>
