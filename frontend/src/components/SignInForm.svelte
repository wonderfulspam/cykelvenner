<script lang="ts">
  import { createEventDispatcher } from 'svelte'

  let email = ''
  let password = ''
  let remember = false
  
  const dispatch = createEventDispatcher()

  function submit() {
    dispatch('submit', {
    email,
    password,
    remember
    })
  }
</script>

<form on:submit|preventDefault={submit}>
  <input type="text" name="login" placeholder="Login" aria-label="Login" autocomplete="nickname" required bind:value={email}>
  <input type="password" name="password" placeholder="Password" aria-label="Password" autocomplete="current-password" required bind:value={password}>
  <fieldset>
    <label for="remember">
      <input type="checkbox" role="switch" id="remember" name="remember" bind:value={remember}>
      Remember me
    </label>
  </fieldset>
  <button type="submit">Login</button>
</form>
