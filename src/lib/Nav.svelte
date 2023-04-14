<script>
    import {Link, navigateTo, router} from 'yrv';
    import {APIserver} from './store'
    import {user, sessionID} from './store'
    let isLoggedIn = false; 

    function updateLoginStatus(){
         if ($user !== "_"){
        isLoggedIn = true; 
    }
    }
    function handleLoginClick(){
        if(!isLoggedIn) {
            navigateTo("/auth/login")
        }else {
            //handle logout
            user.set("_");
            sessionID.set("_")
            isLoggedIn = false;
        }
    }
    $: $user, updateLoginStatus()
    $: $sessionID, updateLoginStatus()
</script>
<div class="row nav-items">
 <div class="active">
 <Link href="/" class="selected"> Home </Link>
 </div>
 <div>
 <a href={`${$APIserver}/message`}> Server Route </a> 
 </div>
 <div>
 <Link href="/common/routeparam?p1=test&p2=3"> Common Route </Link>
 </div>
 <div>
 <Link href="/secure/secret"> Secure Route </Link>
 </div>
 <div>
 <Link href="/about"> About </Link>
 </div>

{#if ($router.path !== "/auth/login") }
 <div>
     <button on:click={handleLoginClick}>{isLoggedIn? "Logout":"Login"}</button>
 </div>
{/if}
</div>
<style>
.nav-items{
    gap: 1em;
    display: flex;
    flex-direction:row;
    justify-content: space-between;
    align-items: center;
}
.active{
    text-decoration: underline;
}
.active:a:link{
    color: red;
}
</style>

