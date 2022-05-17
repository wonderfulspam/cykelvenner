<script>
  import SignUpForm from '../components/SignUpForm.svelte';

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
      <h1>Sign up</h1>
      <h2>We're delighted to meet you.</h2>
    </hgroup>
    <SignUpForm on:submit={handleSubmit} />
  </div>
</article>
