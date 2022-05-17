<script>
  import SignInForm from '../components/SignInForm.svelte';

  let error;

  async function handleSubmit({detail: {email, password}}) {
    const response = await fetch('/api/sign-in', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
      headers: {
          'Content-Type': 'application/json'
      }
    });

    if (!response.ok) {
      error = (await response.json()).message;
      return;
    }

    window.location = '/protected';
  }
</script>

<article class="grid">
  <div>
    <hgroup>
      <h1>Sign in</h1>
      <h2>We're pleased to have you back.</h2>
    </hgroup>
    <SignInForm on:submit={handleSubmit} />
  </div>
</article>
