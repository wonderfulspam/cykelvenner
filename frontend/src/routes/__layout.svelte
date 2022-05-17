<nav>
  <ul>
    <li><a href="/">Home</a></li>
{#if 1 == true }
    <li><a href="/sign-in">Sign in</a></li>
    <li><a href="/sign-up">Sign up</a></li>
{:else }
    <li>Ok!</li>
{/if}
  </ul>
</nav>

<slot></slot>
