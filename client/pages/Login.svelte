<script>
    import {user} from './../store.js';

    let username, password;
    let errorMessage = '';
    async function handleLogin(){
        const res = await fetch('/auth/login',{
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ "username": username, "password": password})
        });
        let loginResponse = await res.json();
        if (loginResponse.result == "error") {
            errorMessage = loginResponse.message;
        }else {
            const res = await fetch('/api/checkuser',{credentials: 'same-origin'});
            let stateResponse = await res.json();
            if (stateResponse.user_id !== '_') {
                user.set(stateResponse.user_id);
            }else 
            {
                user.set('');
            }
        }
    }
</script>

<div class="content">
<div class="columns">
<div class="column"></div>
{#if !$user}
<div class="column">
    {#if errorMessage}
        <div class="notification is-danger">
        {errorMessage}
        </div>
    {/if}
    <div class="field">
    <label class="label">Username</label>
        <div class="control has-icons-left">
            <input class="input" type="username" placeholder="username" bind:value={username}>
                <span class="icon is-small is-left">
                <i class="fas fa-user"></i>
                </span>
        </div>
    </div>
    <div class="field">
    <label class="label">Password</label>
    <div class="control has-icons-left">
        <input class="input" type="password" placeholder="Password" bind:value={password}>
        <span class="icon is-small is-left">
        <i class="fas fa-lock"></i>
        </span>
    </div>
    </div>
    <pre>Enter any <strong>username = password </strong>to login</pre>
    <div class="field">
    <p class="control">
        <button class="button is-success" on:click={handleLogin}>
        Login
        </button>
    </p>
    </div>
</div>
{:else}
 <div class="column">
 Logged in as: {$user} <br>
 Now you may access the <strong>secure area </strong>from the Nav above
 </div>
{/if}
<div class="column"></div>
</div>
</div>