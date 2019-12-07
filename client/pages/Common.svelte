<script>
    import {onMount} from 'svelte';
    import {router} from 'yrv';

    export let contents = "";
    onMount( ()=> getContext())
    async function getContext() {
       let api_path = $router.path.replace('#','/'); 
       const res = await fetch(api_path);
       contents = await res.json(); 
    }
</script>
<h1> Common Route on client and server </h1>
<h3> Data recieved from server </h3>
<p> App : {contents.app} </p>
<p> Version : {contents.version} </p>
<p> Status: {contents.status}</p>


