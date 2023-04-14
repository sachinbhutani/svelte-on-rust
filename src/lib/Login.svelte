<script>
    import {user, APIserver, sessionID} from './store';
 
    let username, password;
    let errorMessage = '';
    let authURL = `${$APIserver}/auth/login`;
    async function handleLogin(){
        try{ 
        
        const res = await fetch(authURL,{
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
            user.set(username);
            sessionID.set(loginResponse.token)
        }
        }catch(e){
            console.log("error connecting to the server!")
            errorMessage = "Error connecting to authentiation server";
        }
    }
    function handleLogout(){
        console.log("call logout API ")
        user.set("_")
        sessionID.set("_")
    }
</script>
{#if ($user == "_")}
<div class="column">
    {#if errorMessage}
        <div class="notification is-danger">
        {errorMessage}
        </div>
    {/if}
    <div>
    <label for="username">Username</label>
        <div class="control has-icons-left">
            <input class="input" type="username" id="username" placeholder="username" bind:value={username}>
        </div>
    </div>
    <div>
    <label for="password">Password</label>
    <div class="control has-icons-left">
        <input class="input" type="password" id="password" placeholder="Password" bind:value={password}>
        <span class="icon is-small is-left">
        <i class="fas fa-lock"></i>
        </span>
    </div>
    </div>
    <pre>Enter any <strong>username = password </strong>to login</pre>
    <div>
    <p class="control">
        <button on:click={handleLogin}>
        Login
        </button>
    </p>
    </div>
</div>
{:else}
 <div class="column">
 Logged in as: {$user} <br>
 <h4> Now you may access the <strong>secured Route </strong>from the Nav above </h4>
 <button on:click={handleLogout}>Logout</button>
 </div>
{/if}
