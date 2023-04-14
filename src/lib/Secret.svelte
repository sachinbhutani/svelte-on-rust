<script>   
import {onMount} from 'svelte'
import {APIserver, sessionID} from './store'
import { router } from 'yrv';
let props; 
let serverpath = "secure/secret"
let errorMessage = ""
onMount( async()=>{
   await getSecretData
})

async function getSecretData(){
     let api_path = `${$APIserver}/${serverpath}`
    try{
        errorMessage = ""
        let res = await fetch(api_path, {
            headers: {
                'x-token': $sessionID
            }
        });
        if (res.status == 401){
            errorMessage = "You are not authorized"
            return;
        }
        props = await res.json()
    }catch(err){
        console.log(err)
    }
    
}

$: $sessionID, getSecretData();
</script> 
{#if errorMessage !== ""}
    <h4 class="error"> {errorMessage}</h4>
{:else}
<h4> Data fetched from server: </h4>
<pre class="rawdata">
{JSON.stringify(props,null, 2)}
</pre>
{/if}
<h4> Data from client Router: </h4>
<pre class="rawdata">
{JSON.stringify($router,null,2)}
</pre>

<style>
.rawdata{
    text-align: left;
}
.error{
    color: red;
}
</style>
