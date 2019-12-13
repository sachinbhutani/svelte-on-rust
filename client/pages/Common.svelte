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
<h3> Data recieved from server on mount: </h3>
<ul>
    <li> App : {contents.app} </li>
    <li> Version : {contents.version} </li>
    <li> Status: {contents.status}</li>
</ul>

