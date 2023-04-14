<script>   
import {onMount} from 'svelte'
import {APIserver} from './store'
import { router } from 'yrv';
let props; 
let serverpath = "message"
onMount( async()=>{
    let api_path = `${$APIserver}/${serverpath}`
    try{
        let res = await fetch(api_path);
        props = await res.json()
    }catch(err){
        console.log(err)
    }
})
</script> 

<h4> Data fetched from server: </h4>
<pre class="rawdata">
{JSON.stringify(props,null, 2)}
</pre>
<h4> Data from client Router: </h4>
<pre class="rawdata">
{JSON.stringify($router,null,2)}
</pre>

<style>
.rawdata{
    text-align: left;
}
</style>
