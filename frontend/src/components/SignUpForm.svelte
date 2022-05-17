<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  let email = ''
  let password = ''
  
  const dispatch = createEventDispatcher()

  function submit() {
    dispatch('submit', {
      email,
      password
    })
  }
</script>

<form on:submit|preventDefault={submit}>
  <input type="text" name="login" placeholder="Login" aria-label="Login" autocomplete="nickname" required bind:value={email}>
  <input type="password" name="password" placeholder="Password" aria-label="Password" autocomplete="current-password" required bind:value={password}>
  <button type="submit">Sign up</button>
</form>
