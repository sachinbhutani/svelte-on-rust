<script>
    import {user} from './store.js';
    import {Link} from 'yrv';

    let isActive = false;
    function toggleMenu(){
        isActive = !isActive;
    }
</script>

<style>
	:global([aria-current]) { font-weight: bold; }
</style>

<nav class="navbar is-primary" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <a class="navbar-item" href="https://github.com/sachinbhutani/svelte-on-rust">
        <strong>Svelte-On-Rust</strong>
        </a>
        <div role="button" class="navbar-burger {isActive? 'is-active' : ''}" aria-label="menu" aria-expanded="false" on:click={toggleMenu}>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
        </div>
    </div>
    <div class="navbar-menu  {isActive? 'is-active' : ''}">
        <div class="navbar-start">
        <Link class="navbar-item" href="/"> Home </Link>
        <Link class="navbar-item" href="#about"> Svelte Route </Link>
            <div class="navbar-item has-dropdown is-hoverable">
                <a class="navbar-link" href="/api">
                Server Routes
                </a>
                    <div class="navbar-dropdown">
                    <a class="navbar-item" href="/api">
                        String
                    </a>
                    <a class="navbar-item" href="/api/message">
                        JSON
                    </a>
                    </div>
            </div>
        <Link class="navbar-item" href="#app/common?p1=test&p2=3"> Common Route </Link>
        {#if $user}<Link class="navbar-item" href="#secure"> Secure Route </Link>{/if}
        <Link class="navbar-item" href="#catch-it"> Catch All </Link>
        </div>
    </div>

    <div class="navbar-end">
    {#if !$user}
        <div class="navbar-item">
        <div class="buttons">
          <Link class="button is-light" href="#login">
            Log in
          </Link>
        </div>
        </div>
    {:else}
        <div class="navbar-item">
            Hi {$user}
        </div>
        <div class="navbar-item">
        <div class="buttons">
          <Link class="button is-danger" href="#logout">
            Log Out
          </Link>
        </div>
        </div>
    {/if}
    </div>

</nav>
